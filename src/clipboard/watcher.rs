use crate::clipboard::{ClipboardContent, ClipboardItem, ClipboardManager};
use crate::error::ClipboardResult;
use log::{debug, error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tokio::time;

/// Canal pour envoyer des notifications de changement du presse-papiers
pub type ClipboardEventSender = mpsc::Sender<ClipboardItem>;
pub type ClipboardEventReceiver = mpsc::Receiver<ClipboardItem>;

/// Surveillance du presse-papiers, exécuté dans un thread tokio séparé
pub struct ClipboardWatcher {
	sender: ClipboardEventSender,
	receiver: Option<ClipboardEventReceiver>,
	last_content: Arc<Mutex<Option<ClipboardContent>>>,
	running: Arc<Mutex<bool>>,
}

impl ClipboardWatcher {
	/// Crée une nouvelle instance du surveillant de presse-papiers
	pub fn new() -> Self {
		let (sender, receiver) = mpsc::channel(100); // Buffer de 100 événements
		Self {
			sender,
			receiver: Some(receiver),
			last_content: Arc::new(Mutex::new(None)),
			running: Arc::new(Mutex::new(false)),
		}
	}

	/// Récupère le récepteur du canal d'événements
	pub fn take_receiver(&mut self) -> Option<ClipboardEventReceiver> {
		self.receiver.take()
	}

	/// Démarre la surveillance du presse-papiers dans une tâche tokio
	pub async fn start(&self) -> ClipboardResult<()> {
		let mut running = self.running.lock().await;
		if *running {
			return Ok(());
		}
		*running = true;
		drop(running);

		let sender = self.sender.clone();
		let last_content = self.last_content.clone();
		let running = self.running.clone();

		tokio::spawn(async move {
			let mut interval = time::interval(Duration::from_millis(500));
			
			while *running.lock().await {
				interval.tick().await;
				
				// Vérifier le contenu du presse-papiers
				match ClipboardManager::new() {
					Ok(mut manager) => {
						match manager.get_current_content() {
							Ok(Some(current_content)) => {
								let mut last = last_content.lock().await;

								// Vérifier si le contenu a changé
								let content_changed = match &*last {
									Some(last_content) => {
										!Self::contents_equal(last_content, &current_content)
									}
									None => true, // Pas de contenu précédent
								};

								if content_changed {
									debug!("Nouveau contenu détecté dans le presse-papiers");
									
									// Mettre à jour le dernier contenu connu
									*last = Some(current_content.clone());
									
									// Notifier les auditeurs
									let item = ClipboardItem::new(current_content);
									if let Err(e) = sender.send(item).await {
										error!("Erreur lors de l'envoi de l'événement: {}", e);
									}
								}
							}
							Ok(None) => {
								debug!("Presse-papiers vide");
							}
							Err(e) => {
								error!("Erreur lors de la lecture du presse-papiers: {}", e);
							}
						}
					}
					Err(e) => {
						error!("Erreur lors de la création du gestionnaire de presse-papiers: {}", e);
					}
				}
			}
			
			info!("Surveillance du presse-papiers arrêtée");
		});

		Ok(())
	}

	/// Arrête la surveillance du presse-papiers
	pub async fn stop(&self) {
		let mut running = self.running.lock().await;
		*running = false;
	}

	/// Compare deux contenus de presse-papiers pour déterminer s'ils sont égaux
	fn contents_equal(a: &ClipboardContent, b: &ClipboardContent) -> bool {
		match (a, b) {
			(ClipboardContent::Text(a_text), ClipboardContent::Text(b_text)) => a_text == b_text,
			(
				ClipboardContent::Image(a_data, a_meta),
				ClipboardContent::Image(b_data, b_meta),
			) => {
				a_meta.width == b_meta.width
					&& a_meta.height == b_meta.height
					&& a_meta.bytes_per_pixel == b_meta.bytes_per_pixel
					&& a_data == b_data
			}
			// Types différents
			_ => false,
		}
	}
}

impl Default for ClipboardWatcher {
	fn default() -> Self {
		Self::new()
	}
}