mod sled_storage;

pub use sled_storage::SledStorage;

use crate::clipboard::ClipboardItem;
use crate::error::ClipboardResult;
use std::path::Path;
use uuid::Uuid;

/// Trait définissant les opérations de stockage
pub trait Storage: Send + Sync {
	/// Initialise le stockage
	fn init(&self) -> ClipboardResult<()>;

	/// Récupère tous les éléments du presse-papiers
	fn get_all_items(&self) -> ClipboardResult<Vec<ClipboardItem>>;

	/// Récupère un élément spécifique par son ID
	fn get_item(&self, id: Uuid) -> ClipboardResult<Option<ClipboardItem>>;

	/// Ajoute un nouvel élément
	fn add_item(&self, item: ClipboardItem) -> ClipboardResult<()>;

	/// Met à jour un élément existant
	fn update_item(&self, item: ClipboardItem) -> ClipboardResult<()>;

	/// Supprime un élément par son ID
	fn remove_item(&self, id: Uuid) -> ClipboardResult<()>;

	/// Supprime tous les éléments sauf ceux épinglés
	fn clear_non_pinned(&self) -> ClipboardResult<()>;

	/// Sauvegarde les données si nécessaire
	fn flush(&self) -> ClipboardResult<()>;
}

/// Crée une instance du moteur de stockage configuré
pub fn create_storage<P: AsRef<Path>>(data_dir: P) -> ClipboardResult<Box<dyn Storage>> {
	let storage = SledStorage::new(data_dir)?;
	storage.init()?;
	Ok(Box::new(storage))
}