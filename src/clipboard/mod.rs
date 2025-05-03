mod watcher;

pub use watcher::ClipboardWatcher;

use crate::error::{ClipboardError, ClipboardResult};
use arboard::{Clipboard, ImageData};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Types d'éléments pouvant être stockés dans le presse-papiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardContent {
	Text(String),
	Image(Vec<u8>, ImageMetadata),
	// À l'avenir, nous pourrions ajouter d'autres types (HTML, fichiers, etc.)
}

/// Métadonnées pour les images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
	pub width: usize,
	pub height: usize,
	pub bytes_per_pixel: usize,
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
			// Les images ne peuvent pas être recherchées par texte
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
					bytes_per_pixel: image.bytes_per_pixel,
				};

				// Conversion en Vec<u8> pour la sérialisation
				Ok(Some(ClipboardContent::Image(image.bytes.to_vec(), metadata)))
			}
			Err(_) => Ok(None), // Aucun contenu reconnu
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
					bytes_per_pixel: metadata.bytes_per_pixel,
					bytes: data.as_slice().into(),
				};
				self.clipboard
					.set_image(image)
					.map_err(|e| ClipboardError::Clipboard(e.to_string()))?;
			}
		}
		Ok(())
	}
}