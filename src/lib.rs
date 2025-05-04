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
	// Utiliser la nouvelle API d'Iced 0.13
	iced::application("Gestionnaire de presse-papiers", 
		ClipboardManagerApp::update, 
		ClipboardManagerApp::view)
		.subscription(ClipboardManagerApp::subscription)
		.theme(ClipboardManagerApp::theme)
		.window(window_settings)
		.run_with(ClipboardManagerApp::new)
}