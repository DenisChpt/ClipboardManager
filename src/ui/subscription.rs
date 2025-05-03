use crate::clipboard::{ClipboardItem, ClipboardWatcher};
use crate::ui::Message;
use iced::Subscription;
use iced::advanced::subscription;
use log::{debug, error, info};
use tokio::sync::mpsc;

/// Crée un abonnement pour surveiller les changements du presse-papiers
pub fn clipboard_subscription() -> Subscription<Message> {
	struct ClipboardWatcherSubscription;

	subscription::channel(
		std::any::TypeId::of::<ClipboardWatcherSubscription>(),
		100,
		|output| async move {
			// Crée et démarre la surveillance du presse-papiers
			let mut watcher = ClipboardWatcher::new();
			let receiver = watcher.take_receiver().expect("Impossible d'obtenir le récepteur");
			
			info!("Démarrage de la surveillance du presse-papiers");
			watcher.start().await.expect("Erreur démarrage surveillance presse-papiers");
			
			// Traiter les événements du presse-papiers
			process_clipboard_events(receiver, output).await;
			
			// Ne devrait jamais atteindre ce point sauf si la surveillance s'arrête
			info!("Surveillance du presse-papiers terminée");
		},
	)
}

/// Traite les événements du presse-papiers et les envoie à l'interface
async fn process_clipboard_events(
	mut receiver: mpsc::Receiver<ClipboardItem>,
	output: mpsc::Sender<Message>,
) {
	while let Some(item) = receiver.recv().await {
		debug!("Nouvel élément dans le presse-papiers détecté");
		
		// Envoyer l'événement à l'interface
		if let Err(e) = output.send(Message::NewClipboardItem(item)).await {
			error!("Erreur envoi message UI: {}", e);
			break;
		}
	}
}