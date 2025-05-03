use crate::clipboard::ClipboardWatcher;
use crate::ui::Message;
use iced::Subscription;
use iced::futures::SinkExt;
use log::{debug, error, info};

/// Crée un abonnement pour surveiller les changements du presse-papiers
pub fn clipboard_subscription() -> Subscription<Message> {
	// Dans Iced 0.13, on utilise Subscription::run directement
	Subscription::run("clipboard_watcher", async move |mut output| {
		// Crée et démarre la surveillance du presse-papiers
		let mut watcher = ClipboardWatcher::new();
		let mut receiver = watcher.take_receiver().expect("Impossible d'obtenir le récepteur");
		
		info!("Démarrage de la surveillance du presse-papiers");
		watcher.start().await.expect("Erreur démarrage surveillance presse-papiers");
		
		// Traiter les événements du presse-papiers
		while let Some(item) = receiver.recv().await {
			debug!("Nouvel élément dans le presse-papiers détecté");
			
			// Envoyer l'événement à l'interface
			if let Err(e) = output.send(Message::NewClipboardItem(item)).await {
				error!("Erreur envoi message UI: {}", e);
				break;
			}
		}
		
		// Ne devrait jamais atteindre ce point sauf si la surveillance s'arrête
		info!("Surveillance du presse-papiers terminée");
	})
}