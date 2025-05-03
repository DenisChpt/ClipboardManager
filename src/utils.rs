use crate::error::{ClipboardError, ClipboardResult};
use std::path::Path;
use std::fs;
use image::{DynamicImage, GenericImageView};

/// Vérifie si un dossier existe et le crée si nécessaire
pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> ClipboardResult<()> {
	let path = path.as_ref();
	if !path.exists() {
		fs::create_dir_all(path)
			.map_err(|e| ClipboardError::Io(e))?;
	}
	Ok(())
}

/// Redimensionne une image pour l'affichage dans l'interface
pub fn resize_image(img: &DynamicImage, max_width: u32, max_height: u32) -> DynamicImage {
	let (width, height) = img.dimensions();
	
	// Si l'image est déjà plus petite que les dimensions maximales, ne pas la redimensionner
	if width <= max_width && height <= max_height {
		return img.clone();
	}
	
	// Calcul des proportions
	let width_ratio = max_width as f32 / width as f32;
	let height_ratio = max_height as f32 / height as f32;
	
	// Utiliser le ratio le plus petit pour conserver les proportions
	let ratio = width_ratio.min(height_ratio);
	
	// Nouvelles dimensions
	let new_width = (width as f32 * ratio) as u32;
	let new_height = (height as f32 * ratio) as u32;
	
	// Redimensionnement
	img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

/// Récupère les informations système
pub fn get_system_info() -> String {
	let os = std::env::consts::OS;
	let arch = std::env::consts::ARCH;
	let family = std::env::consts::FAMILY;
	
	format!("OS: {}, Architecture: {}, Famille: {}", os, arch, family)
}

/// Nettoie une chaîne de caractères pour l'affichage
pub fn sanitize_text(text: &str, max_length: usize) -> String {
	let trimmed = text.trim();
	
	if trimmed.len() <= max_length {
		return trimmed.to_string();
	}
	
	// Troncature avec ellipse
	format!("{}...", &trimmed[..max_length - 3])
}

/// Vérifie que les ressources sont disponibles et crée les dossiers si nécessaire
pub fn ensure_resources_available() -> ClipboardResult<()> {
	// Vérifier le dossier des icônes
	let icons_dir = Path::new("assets/icons");
	ensure_dir_exists(icons_dir)?;
	
	// Vérifier la présence de chaque icône
	let icon_files = [
		"sun.svg",
		"moon.svg",
		"trash.svg",
		"pin.svg",
		"pinned.svg",
		"use.svg",
	];
	
	for icon in &icon_files {
		let icon_path = icons_dir.join(icon);
		if !icon_path.exists() {
			return Err(ClipboardError::Unexpected(
				format!("L'icône {} est manquante", icon)
			));
		}
	}
	
	Ok(())
}