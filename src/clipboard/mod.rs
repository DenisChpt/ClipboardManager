mod watcher;

pub use watcher::ClipboardWatcher;

use crate::error::{ClipboardError, ClipboardResult};
use arboard::{Clipboard, ImageData};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;
use std::process::{Command, Stdio};

/// Types d'éléments pouvant être stockés dans le presse-papiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardContent {
	Text(String),
	Image(Vec<u8>, ImageMetadata),
}

/// Métadonnées pour les images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
	pub width: usize,
	pub height: usize,
}

/// Un élément du presse-papiers avec ses métadonnées
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
	pub id: Uuid,
	pub content: ClipboardContent,
	pub timestamp: DateTime<Utc>,
	pub pinned: bool,
}

impl ClipboardItem {
	/// Crée un nouvel élément de presse-papiers à partir du contenu
	pub fn new(content: ClipboardContent) -> Self {
		Self {
			id: Uuid::new_v4(),
			content,
			timestamp: Utc::now(),
			pinned: false,
		}
	}

	/// Vérifie si l'élément contient du texte correspondant à la recherche
	pub fn matches_search(&self, query: &str) -> bool {
		if query.is_empty() {
			return true;
		}

		match &self.content {
			ClipboardContent::Text(text) => text.to_lowercase().contains(&query.to_lowercase()),
			ClipboardContent::Image(_, _) => false,
		}
	}
}

impl fmt::Display for ClipboardItem {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.content {
			ClipboardContent::Text(text) => {
				let preview = if text.len() > 50 {
					format!("{}...", &text[..47])
				} else {
					text.clone()
				};
				write!(f, "{}", preview)
			}
			ClipboardContent::Image(_, metadata) => {
				write!(f, "Image {}x{}", metadata.width, metadata.height)
			}
		}
	}
}

/// Gestion des opérations de presse-papiers
pub struct ClipboardManager {
	clipboard: Clipboard,
}

impl ClipboardManager {
	/// Crée une nouvelle instance du gestionnaire de presse-papiers
	pub fn new() -> ClipboardResult<Self> {
		let clipboard = Clipboard::new().map_err(|e| ClipboardError::Clipboard(e.to_string()))?;
		Ok(Self { clipboard })
	}

	/// Récupère le contenu actuel du presse-papiers
	pub fn get_current_content(&mut self) -> ClipboardResult<Option<ClipboardContent>> {
		// D'abord, essayons de récupérer le texte
		if let Ok(text) = self.clipboard.get_text() {
			if !text.is_empty() {
				return Ok(Some(ClipboardContent::Text(text)));
			}
		}

		// Si ce n'est pas du texte, essayons une image
		match self.clipboard.get_image() {
			Ok(image) => {
				let metadata = ImageMetadata {
					width: image.width,
					height: image.height,
				};

				// Conversion en Vec<u8> pour la sérialisation
				Ok(Some(ClipboardContent::Image(image.bytes.to_vec(), metadata)))
			}
			Err(_) => Ok(None),
		}
	}

	/// Place un élément dans le presse-papiers
	pub fn set_content(&mut self, item: &ClipboardItem) -> ClipboardResult<()> {
		match &item.content {
			ClipboardContent::Text(text) => {
				self.clipboard
					.set_text(text.clone())
					.map_err(|e| ClipboardError::Clipboard(e.to_string()))?;
			}
			ClipboardContent::Image(data, metadata) => {
				let image = ImageData {
					width: metadata.width,
					height: metadata.height,
					bytes: data.as_slice().into(),
				};
				self.clipboard
					.set_image(image)
					.map_err(|e| ClipboardError::Clipboard(e.to_string()))?;
			}
		}
		Ok(())
	}

	/// Vérifie si ydotool est disponible
	pub fn check_ydotool_available() -> bool {
		Command::new("ydotool")
			.arg("--version")
			.output()
			.map(|output| output.status.success())
			.unwrap_or(false)
	}

	/// Colle directement le contenu dans la fenêtre active
	pub async fn paste_to_active_window(&mut self, item: &ClipboardItem) -> ClipboardResult<()> {
		// D'abord, mettre le contenu dans le presse-papiers si c'est une image
		match &item.content {
			ClipboardContent::Image(_, _) => {
				self.set_content(item)?;
			}
			_ => {} // Pour le texte, on laisse ydotool type gérer
		}
		
		// Petite pause pour s'assurer que l'environnement est prêt
		tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
		
		// Pour AZERTY, utiliser la commande appropriée
		if cfg!(target_os = "linux") {
			match &item.content {
				ClipboardContent::Text(text) => {
					// Pour le texte, utiliser la commande type qui gère l'AZERTY automatiquement
					let result = Command::new("ydotool")
						.arg("type")
						.arg("--")
						.arg(text)
						.stdin(Stdio::null())
						.output();
						
					match result {
						Ok(output) => {
							if output.status.success() {
								return Ok(());
							} else {
								log::error!("ydotool type failed: {}", String::from_utf8_lossy(&output.stderr));
							}
						}
						Err(e) => {
							log::error!("Error executing ydotool type: {}", e);
						}
					}
				}
				ClipboardContent::Image(_, _) => {
					// Pour les images, utiliser Ctrl+V
					let result = Command::new("ydotool")
						.args(&["key", "29:1", "47:1", "47:0", "29:0"])  // Ctrl+V
						.output();
						
					match result {
						Ok(output) => {
							if output.status.success() {
								return Ok(());
							} else {
								log::error!("ydotool key failed: {}", String::from_utf8_lossy(&output.stderr));
							}
						}
						Err(e) => {
							log::error!("Error executing ydotool key: {}", e);
						}
					}
				}
			}
			
			// Si ydotool échoue, retourner une erreur appropriée
			return Err(ClipboardError::Unexpected(
				"Erreur lors du collage avec ydotool. Assurez-vous que ydotool est installé et que le service ydotool est en cours d'exécution."
					.to_string()
			));
		}
		
		// Pour les autres OS, on ne fait rien (set_content a déjà mis dans le presse-papiers)
		Ok(())
	}
}
