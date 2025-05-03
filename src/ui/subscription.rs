use crate::clipboard::ClipboardWatcher;
use crate::ui::Message;
use iced::Subscription;
use iced::futures::stream::{self};
use log::{debug, info};

/// Crée un abonnement pour surveiller les changements du presse-papiers
pub fn clipboard_subscription() -> Subscription<Message> {
	// Version avec stream::unfold qui est compatible avec la version actuelle d'Iced
	Subscription::run(|| {
		// Créer une fonction qui retourne un Stream
		let stream = stream::unfold(
			ClipboardWatcherState::Starting,
			move |state| async move {
				match state {
					ClipboardWatcherState::Starting => {
						// Créer et initialiser la surveillance
						let mut watcher = ClipboardWatcher::new();
						let receiver = watcher.take_receiver().expect("Impossible d'obtenir le récepteur");
						
						info!("Démarrage de la surveillance du presse-papiers");
						watcher.start().await.expect("Erreur démarrage surveillance presse-papiers");
						
						// Passer à l'état Watching avec le receiver
						Some((
							Message::None, 
							ClipboardWatcherState::Watching(watcher, receiver)
						))
					},
					ClipboardWatcherState::Watching(watcher, mut receiver) => {
						// Attendre le prochain événement du presse-papiers
						if let Some(item) = receiver.recv().await {
							debug!("Nouvel élément dans le presse-papiers détecté");
							
							// Émettre un événement et continuer d'écouter
							Some((
								Message::NewClipboardItem(item),
								ClipboardWatcherState::Watching(watcher, receiver)
							))
						} else {
							// Le canal a été fermé, terminer la surveillance
							info!("Surveillance du presse-papiers terminée");
							None
						}
					}
				}
			}
		);
		
		// Retourner le stream directement
		stream
	})
}

/// État de la surveillance du presse-papiers
enum ClipboardWatcherState {
	Starting,
	Watching(ClipboardWatcher, tokio::sync::mpsc::Receiver<crate::clipboard::ClipboardItem>),
}