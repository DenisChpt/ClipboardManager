use crate::clipboard::ClipboardItem;
use crate::error::{ClipboardError, ClipboardResult};
use crate::storage::Storage;
use log::{debug, error, info};
use serde_json;
use sled::{Config, Db};
use std::path::Path;
use uuid::Uuid;

/// Implémentation du stockage utilisant Sled comme backend
pub struct SledStorage {
	db: Db,
}

impl SledStorage {
	/// Crée une nouvelle instance de stockage Sled
	pub fn new<P: AsRef<Path>>(data_dir: P) -> ClipboardResult<Self> {
		let config = Config::new()
			.path(data_dir)
			.cache_capacity(64 * 1024 * 1024) // 64MB de cache
			.flush_every_ms(Some(1000)); // Écriture sur disque toutes les secondes

		let db = config
			.open()
			.map_err(|e| ClipboardError::Storage(format!("Erreur ouverture Sled: {}", e)))?;

		Ok(Self { db })
	}

	/// Convertit un ID UUID en clé pour Sled
	fn id_to_key(id: Uuid) -> Vec<u8> {
		id.as_bytes().to_vec()
	}

	/// Convertit un élément en valeur pour Sled
	fn item_to_value(item: &ClipboardItem) -> ClipboardResult<Vec<u8>> {
		serde_json::to_vec(item).map_err(|e| e.into())
	}

	/// Convertit une valeur Sled en élément
	fn value_to_item(value: &[u8]) -> ClipboardResult<ClipboardItem> {
		serde_json::from_slice(value).map_err(|e| e.into())
	}
}

impl Storage for SledStorage {
	fn init(&self) -> ClipboardResult<()> {
		debug!("Initialisation du stockage Sled");
		Ok(())
	}

	fn get_all_items(&self) -> ClipboardResult<Vec<ClipboardItem>> {
		let mut items = Vec::new();

		for result in self.db.iter() {
			match result {
				Ok((_, value)) => {
					match Self::value_to_item(&value) {
						Ok(item) => items.push(item),
						Err(e) => error!("Erreur désérialisation élément: {}", e),
					}
				}
				Err(e) => {
					error!("Erreur lecture base de données: {}", e);
				}
			}
		}

		// Trier par horodatage (le plus récent en premier)
		items.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

		Ok(items)
	}

	fn get_item(&self, id: Uuid) -> ClipboardResult<Option<ClipboardItem>> {
		let key = Self::id_to_key(id);

		match self.db.get(key) {
			Ok(Some(value)) => {
				let item = Self::value_to_item(&value)?;
				Ok(Some(item))
			}
			Ok(None) => Ok(None),
			Err(e) => Err(ClipboardError::Storage(format!("Erreur lecture élément: {}", e))),
		}
	}

	fn add_item(&self, item: ClipboardItem) -> ClipboardResult<()> {
		let key = Self::id_to_key(item.id);
		let value = Self::item_to_value(&item)?;

		self.db
			.insert(key, value)
			.map_err(|e| ClipboardError::Storage(format!("Erreur ajout élément: {}", e)))?;

		debug!("Élément ajouté: {}", item.id);
		Ok(())
	}

	fn update_item(&self, item: ClipboardItem) -> ClipboardResult<()> {
		let key = Self::id_to_key(item.id);
		let value = Self::item_to_value(&item)?;

		self.db
			.insert(key, value)
			.map_err(|e| ClipboardError::Storage(format!("Erreur mise à jour élément: {}", e)))?;

		debug!("Élément mis à jour: {}", item.id);
		Ok(())
	}

	fn remove_item(&self, id: Uuid) -> ClipboardResult<()> {
		let key = Self::id_to_key(id);

		self.db
			.remove(key)
			.map_err(|e| ClipboardError::Storage(format!("Erreur suppression élément: {}", e)))?;

		debug!("Élément supprimé: {}", id);
		Ok(())
	}

	fn clear_non_pinned(&self) -> ClipboardResult<()> {
		// Récupérer tous les éléments
		let items = self.get_all_items()?;
		let mut removed = 0;

		// Supprimer tous les éléments non épinglés
		for item in items {
			if !item.pinned {
				self.remove_item(item.id)?;
				removed += 1;
			}
		}

		info!("{} éléments non épinglés supprimés", removed);
		Ok(())
	}

	fn flush(&self) -> ClipboardResult<()> {
		self.db
			.flush()
			.map_err(|e| ClipboardError::Storage(format!("Erreur flush stockage: {}", e)))?;
		
		debug!("Données synchronisées sur le disque");
		Ok(())
	}
}