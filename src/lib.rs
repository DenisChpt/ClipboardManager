pub mod app;
pub mod clipboard;
pub mod config;
pub mod error;
pub mod storage;
pub mod ui;
pub mod utils;

use app::ClipboardManagerApp;
use iced::{window, Result};

/// Fonction pour exécuter l'application en évitant les problèmes de durée de vie
pub fn run(window_settings: window::Settings) -> Result {
	// Création de l'application avec la nouvelle API d'iced 0.13
	// Utilisation de la fonction view statique plutôt que la méthode
	iced::application("Gestionnaire de presse-papiers", 
		ClipboardManagerApp::update, 
		ClipboardManagerApp::view) // Correction ici: utiliser la fonction statique
		.theme(ClipboardManagerApp::theme)
		.subscription(ClipboardManagerApp::subscription)
		.window(window_settings)
		.run_with(ClipboardManagerApp::new)
}