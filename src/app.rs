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
		
		// Pour éviter l'erreur de propriété avec app.storage.clone()
		let storage_clone = app.storage.clone();
		
		// Charger les éléments au démarrage
		(app, Task::perform(Self::load_items(storage_clone), Message::ItemsLoaded))
	}

	/// Met à jour l'état de l'application en fonction du message reçu
	pub fn update(app: &mut Self, message: Message) -> Task<Message> {
		match message {
			Message::ItemsLoaded(items) => {
				app.items = items;
				Task::none()
			}
			Message::NewClipboardItem(item) => {
				let storage = app.storage.clone();
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
			Message::SelectItem(id) => {
				let item = app.items.iter().find(|item| item.id == id).cloned();
				if let Some(item) = item {
					let clipboard_manager = app.clipboard_manager.clone();
					
					Task::perform(
						async move {
							let mut manager = clipboard_manager.lock().await;
							manager.set_content(&item)?;
							Ok(())
						},
						|result: ClipboardResult<()>| {
							if let Err(e) = result {
								error!("Erreur copie élément: {}", e);
							}
							Message::None
						},
					)
				} else {
					Task::none()
				}
			}
			Message::PinItem(id) => {
				let item = app.items.iter().find(|item| item.id == id).cloned();
				if let Some(mut item) = item {
					item.pinned = !item.pinned;
					let storage = app.storage.clone();
					
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
				let storage = app.storage.clone();
				
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
				let storage = app.storage.clone();
				
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
				app.config.theme = theme;
				let config = app.config.clone();
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
				app.search_query = query;
				Task::none()
			}
			Message::ReloadItems => {
				let storage = app.storage.clone();
				Task::perform(Self::load_items(storage), Message::ItemsLoaded)
			}
			Message::None => Task::none(),
		}
	}

	/// Affiche l'interface utilisateur
	/// Modifié pour être une fonction statique (sans &self)
	pub fn view(app: &Self) -> Element<Message> {
		// Filtrer les éléments selon la recherche
		let filtered_items = if app.search_query.is_empty() {
			app.items.clone()
		} else {
			app.items
				.iter()
				.filter(|item| item.matches_search(&app.search_query))
				.cloned()
				.collect()
		};
		
		// on retourne directement la vue pour éviter l'erreur de référence locale
		crate::ui::view(app.ui_state.clone(), filtered_items, app.search_query.clone(), app.config.theme)
	}

	/// Abonnements aux événements externes
	pub fn subscription(_app: &Self) -> Subscription<Message> {
		// Subscribe to clipboard changes
		crate::ui::clipboard_subscription()
	}

	/// Thème de l'application
	pub fn theme(app: &Self) -> IcedTheme {
		match app.config.theme {
			Theme::Light => IcedTheme::Light,
			Theme::Dark => IcedTheme::Dark,
			// Utiliser le prédicat de la plateforme pour déterminer le thème système
			Theme::System => {
				if cfg!(target_os = "macos") {
					// Sur macOS, on peut détecter le mode sombre
					if is_macos_dark_mode() {
						IcedTheme::Dark
					} else {
						IcedTheme::Light
					}
				} else {
					// Sur les autres plateformes, par défaut light
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