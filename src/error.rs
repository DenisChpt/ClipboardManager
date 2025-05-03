use thiserror::Error;

/// Erreurs spécifiques au gestionnaire de presse-papiers
#[derive(Debug, Error)]
pub enum ClipboardError {
	#[error("Erreur d'entrée/sortie: {0}")]
	Io(#[from] std::io::Error),

	#[error("Erreur de sérialisation: {0}")]
	Serialization(#[from] serde_json::Error),

	#[error("Erreur presse-papiers: {0}")]
	Clipboard(String),

	#[error("Erreur de stockage: {0}")]
	Storage(String),

	#[error("Erreur UI: {0}")]
	Ui(String),

	#[error("Erreur de configuration: {0}")]
	Config(String),

	#[error("Erreur inattendue: {0}")]
	Unexpected(String),
}

/// Alias de résultat pour les opérations du gestionnaire de presse-papiers
pub type ClipboardResult<T> = Result<T, ClipboardError>;