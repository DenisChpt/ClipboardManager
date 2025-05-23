use crate::clipboard::{ClipboardItem, ClipboardManager};
use crate::config::{get_default_config_path, Config, Theme};
use crate::error::ClipboardResult;
use crate::storage::{create_storage, Storage};
use crate::ui::Message;
use iced::{Element, Subscription, Task, Theme as IcedTheme};
use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::Mutex;

/// État de l'application
#[derive(Clone)]
pub struct ClipboardManagerApp {
	/// Configuration de l'application
	pub config: Config,
	
	/// Interface utilisateur
	ui_state: crate::ui::State,
	
	/// Stockage des éléments
	storage: Arc<Mutex<Box<dyn Storage>>>,
	
	/// Gestionnaire du presse-papiers
	clipboard_manager: Arc<Mutex<ClipboardManager>>,
	
	/// Éléments du presse-papiers
	items: Vec<ClipboardItem>,
	
	/// Terme de recherche
	search_query: String,
}

impl ClipboardManagerApp {
	/// Crée une nouvelle instance de l'application
	pub fn new() -> (Self, Task<Message>) {
		let config_path = get_default_config_path();
		
		// Charger la configuration
		let config = match Config::load(&config_path) {
			Ok(config) => {
				info!("Configuration chargée depuis {}", config_path.display());
				config
			}
			Err(e) => {
				error!("Erreur chargement configuration: {}. Utilisation des valeurs par défaut.", e);
				Config::default()
			}
		};
		
		// Créer le dossier de données s'il n'existe pas
		match std::fs::create_dir_all(&config.data_dir) {
			Ok(_) => debug!("Dossier de données: {}", config.data_dir.display()),
			Err(e) => error!("Erreur création dossier données: {}", e),
		}
		
		// Initialiser le stockage
		let storage = match create_storage(&config.data_dir) {
			Ok(storage) => {
				info!("Stockage initialisé dans {}", config.data_dir.display());
				storage
			}
			Err(e) => {
				error!("Erreur initialisation stockage: {}. Arrêt de l'application.", e);
				panic!("Impossible d'initialiser le stockage");
			}
		};
		
		// Initialiser le gestionnaire de presse-papiers
		let clipboard_manager = match ClipboardManager::new() {
			Ok(manager) => {
				info!("Gestionnaire de presse-papiers initialisé");
				manager
			}
			Err(e) => {
				error!("Erreur initialisation gestionnaire de presse-papiers: {}. Arrêt de l'application.", e);
				panic!("Impossible d'initialiser le gestionnaire de presse-papiers");
			}
		};
		
		let app = Self {
			config,
			ui_state: crate::ui::State::default(),
			storage: Arc::new(Mutex::new(storage)),
			clipboard_manager: Arc::new(Mutex::new(clipboard_manager)),
			items: Vec::new(),
			search_query: String::new(),
		};
		
		let storage_clone = app.storage.clone();
		
		// Charger les éléments au démarrage
		(app, Task::perform(Self::load_items(storage_clone), Message::ItemsLoaded))
	}

	/// Met à jour l'état de l'application en fonction du message reçu
	pub fn update(&mut self, message: Message) -> Task<Message> {
		match message {
			Message::ItemsLoaded(items) => {
				self.items = items;
				Task::none()
			}
			Message::NewClipboardItem(item) => {
				let storage = self.storage.clone();
				Task::perform(
					async move {
						let storage = storage.lock().await;
						storage.add_item(item.clone())?;
						storage.flush()?;
						Ok(())
					},
					|result: ClipboardResult<()>| {
						if let Err(e) = result {
							error!("Erreur sauvegarde élément: {}", e);
						}
						Message::ReloadItems
					},
				)
			}
			Message::UseItem(id) => {
				let item = self.items.iter().find(|item| item.id == id).cloned();
				if let Some(item) = item {
					let clipboard_manager = self.clipboard_manager.clone();
					
					Task::perform(
						async move {
							let mut manager = clipboard_manager.lock().await;
							// Coller directement le contenu
							manager.paste_to_active_window(&item).await?;
							Ok(())
						},
						|result: ClipboardResult<()>| {
							if let Err(e) = result {
								error!("Erreur lors du collage: {}", e);
							}
							Message::None
						},
					)
				} else {
					Task::none()
				}
			}
			Message::PinItem(id) => {
				let item = self.items.iter().find(|item| item.id == id).cloned();
				if let Some(mut item) = item {
					item.pinned = !item.pinned;
					let storage = self.storage.clone();
					
					Task::perform(
						async move {
							let storage = storage.lock().await;
							storage.update_item(item)?;
							storage.flush()?;
							Ok(())
						},
						|result: ClipboardResult<()>| {
							if let Err(e) = result {
								error!("Erreur épinglage élément: {}", e);
							}
							Message::ReloadItems
						},
					)
				} else {
					Task::none()
				}
			}
			Message::RemoveItem(id) => {
				let storage = self.storage.clone();
				
				Task::perform(
					async move {
						let storage = storage.lock().await;
						storage.remove_item(id)?;
						storage.flush()?;
						Ok(())
					},
					|result: ClipboardResult<()>| {
						if let Err(e) = result {
							error!("Erreur suppression élément: {}", e);
						}
						Message::ReloadItems
					},
				)
			}
			Message::ClearItems => {
				let storage = self.storage.clone();
				
				Task::perform(
					async move {
						let storage = storage.lock().await;
						storage.clear_non_pinned()?;
						storage.flush()?;
						Ok(())
					},
					|result: ClipboardResult<()>| {
						if let Err(e) = result {
							error!("Erreur suppression éléments: {}", e);
						}
						Message::ReloadItems
					},
				)
			}
			Message::SetTheme(theme) => {
				self.config.theme = theme;
				let config = self.config.clone();
				let config_path = get_default_config_path();
				
				Task::perform(
					async move {
						config.save(config_path)?;
						Ok(())
					},
					|result: ClipboardResult<()>| {
						if let Err(e) = result {
							error!("Erreur sauvegarde configuration: {}", e);
						}
						Message::None
					},
				)
			}
			Message::SearchChanged(query) => {
				self.search_query = query;
				Task::none()
			}
			Message::ReloadItems => {
				let storage = self.storage.clone();
				Task::perform(Self::load_items(storage), Message::ItemsLoaded)
			}
			Message::NavigateUp => {
				if !self.items.is_empty() {
					let current = self.ui_state.selected_index;
					self.ui_state.selected_index = if current == 0 {
						self.items.len() - 1
					} else {
						current - 1
					};
				}
				Task::none()
			}
			Message::NavigateDown => {
				if !self.items.is_empty() {
					let current = self.ui_state.selected_index;
					self.ui_state.selected_index = if current == self.items.len() - 1 {
						0
					} else {
						current + 1
					};
				}
				Task::none()
			}
			Message::UseSelected => {
				if let Some(item) = self.items.get(self.ui_state.selected_index) {
					let item_id = item.id;
					self.update(Message::UseItem(item_id))
				} else {
					Task::none()
				}
			}
			Message::None => Task::none(),
		}
	}

	/// Affiche l'interface utilisateur
	pub fn view(&self) -> Element<Message> {
		// Filtrer les éléments selon la recherche
		let filtered_items = if self.search_query.is_empty() {
			self.items.clone()
		} else {
			self.items
				.iter()
				.filter(|item| item.matches_search(&self.search_query))
				.cloned()
				.collect()
		};
		
		// Utiliser une vue avec le theme léger pour éviter les problèmes de lifetime
		crate::ui::view(self.ui_state.clone(), filtered_items, self.search_query.clone(), self.config.theme, self.theme())
	}

	/// Abonnements aux événements externes
	pub fn subscription(_app: &Self) -> Subscription<Message> {
		Subscription::batch([
			crate::ui::clipboard_subscription(),
			crate::ui::keyboard_subscription(),
		])
	}

	/// Thème de l'application
	pub fn theme(&self) -> IcedTheme {
		match self.config.theme {
			Theme::Light => IcedTheme::Light,
			Theme::Dark => IcedTheme::Dark,
			Theme::System => {
				if cfg!(target_os = "macos") {
					if is_macos_dark_mode() {
						IcedTheme::Dark
					} else {
						IcedTheme::Light
					}
				} else {
					IcedTheme::Light
				}
			}
		}
	}

	/// Charge les éléments depuis le stockage
	async fn load_items(storage: Arc<Mutex<Box<dyn Storage>>>) -> Vec<ClipboardItem> {
		let storage = storage.lock().await;
		match storage.get_all_items() {
			Ok(items) => items,
			Err(e) => {
				error!("Erreur chargement éléments: {}", e);
				Vec::new()
			}
		}
	}
}

/// Détecte si macOS est en mode sombre
#[cfg(target_os = "macos")]
fn is_macos_dark_mode() -> bool {
	use std::process::Command;
	
	let output = Command::new("defaults")
		.args(&["read", "-g", "AppleInterfaceStyle"])
		.output()
		.unwrap_or_else(|_| Default::default());
		
	String::from_utf8_lossy(&output.stdout).trim() == "Dark"
}

#[cfg(not(target_os = "macos"))]
fn is_macos_dark_mode() -> bool {
	false
}
