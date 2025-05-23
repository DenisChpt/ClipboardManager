use anyhow::Result;
use env_logger::Env;
use iced::{window, Size};
use log::{error, info};

fn main() -> Result<()> {
	// Initialisation du logger
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
	info!("Démarrage de ClipboardManager");

	// Vérifier la disponibilité des ressources
	match clipboard_manager::utils::ensure_resources_available() {
		Ok(_) => info!("Ressources (icônes) vérifiées avec succès"),
		Err(e) => {
			error!("Erreur lors de la vérification des ressources: {}", e);
			error!("Assurez-vous que le dossier assets/icons contient tous les fichiers SVG nécessaires");
			return Err(anyhow::anyhow!("Ressources manquantes: {}", e));
		}
	}

	// Configuration de l'application
	let window_settings = window::Settings {
		size: Size::new(400.0, 500.0),
		position: window::Position::Centered,
		min_size: Some(Size::new(300.0, 400.0)),
		resizable: true,
		decorations: true,
		transparent: true,
		..window::Settings::default()
	};

	// Version simplifiée pour éviter les problèmes de durée de vie
	let run_result = clipboard_manager::run(window_settings);

	// Gérer le résultat
	match run_result {
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