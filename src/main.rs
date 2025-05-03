use anyhow::Result;
use clipboard_manager::app::ClipboardManagerApp;
use env_logger::Env;
use iced::{Application, Settings};
use log::{error, info};

fn main() -> Result<()> {
	// Initialisation du logger
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
	info!("Démarrage de ClipboardManager");

	// Configuration de l'application Iced
	let settings = Settings {
		window: iced::window::Settings {
			size: (400, 500),
			position: iced::window::Position::Centered,
			min_size: Some((300, 400)),
			max_size: None,
			visible: true,
			resizable: true,
			decorations: true,
			transparent: true,
			always_on_top: true,
			..Default::default()
		},
		// Activer la gestion des icônes SVG
		default_font: None,
		default_text_size: 16.0,
		// Fermer lors de la demande
		exit_on_close_request: true,
		..Default::default()
	};

	// Lancement de l'application
	match ClipboardManagerApp::run(settings) {
		Ok(_) => {
			info!("Application terminée avec succès");
			Ok(())
		}
		Err(e) => {
			error!("Erreur lors de l'exécution de l'application: {}", e);
			Err(anyhow::anyhow!("Erreur d'exécution: {}", e))
		}
	}
}