use crate::error::{ClipboardError, ClipboardResult};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration de l'application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	/// Taille maximale de l'historique du presse-papiers
	pub max_history_size: usize,
	
	/// Durée de conservation des éléments (en jours)
	pub retention_days: u32,
	
	/// Intervalle de vérification du presse-papiers (en millisecondes)
	pub check_interval_ms: u64,
	
	/// Thème de l'interface (clair ou sombre)
	pub theme: Theme,
	
	/// Toujours afficher en haut
	pub always_on_top: bool,
	
	/// Chemin vers le dossier de données
	pub data_dir: PathBuf,
}

/// Thèmes disponibles
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Theme {
	Light,
	Dark,
	System,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			max_history_size: 100,
			retention_days: 30,
			check_interval_ms: 500,
			theme: Theme::System,
			always_on_top: true,
			data_dir: get_default_data_dir(),
		}
	}
}

impl Config {
	/// Charge la configuration depuis un fichier
	pub fn load<P: AsRef<Path>>(path: P) -> ClipboardResult<Self> {
		let path = path.as_ref();
		
		// Si le fichier n'existe pas, créer la configuration par défaut
		if !path.exists() {
			let config = Config::default();
			config.save(path)?;
			return Ok(config);
		}
		
		// Charger la configuration
		let content = fs::read_to_string(path)
			.map_err(|e| ClipboardError::Config(format!("Erreur lecture config: {}", e)))?;
			
		let config: Config = serde_json::from_str(&content)
			.map_err(|e| ClipboardError::Config(format!("Erreur parse config: {}", e)))?;
			
		Ok(config)
	}
	
	/// Sauvegarde la configuration dans un fichier
	pub fn save<P: AsRef<Path>>(&self, path: P) -> ClipboardResult<()> {
		let path = path.as_ref();
		
		// Créer le dossier parent s'il n'existe pas
		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent)
				.map_err(|e| ClipboardError::Config(format!("Erreur création dossier config: {}", e)))?;
		}
		
		// Sérialiser et sauvegarder
		let content = serde_json::to_string_pretty(self)
			.map_err(|e| ClipboardError::Config(format!("Erreur sérialisation config: {}", e)))?;
			
		fs::write(path, content)
			.map_err(|e| ClipboardError::Config(format!("Erreur écriture config: {}", e)))?;
			
		info!("Configuration sauvegardée: {}", path.display());
		Ok(())
	}
}

/// Détermine le chemin par défaut pour le dossier de données
fn get_default_data_dir() -> PathBuf {
	let mut path = dirs::data_local_dir()
		.unwrap_or_else(|| PathBuf::from("."));
		
	path.push("clipboard-manager");
	path
}

/// Détermine le chemin par défaut pour le fichier de configuration
pub fn get_default_config_path() -> PathBuf {
	let mut path = dirs::config_dir()
		.unwrap_or_else(|| PathBuf::from("."));
		
	path.push("clipboard-manager");
	path.push("config.json");
	path
}