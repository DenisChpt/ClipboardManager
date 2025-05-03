use std::fs;
use std::path::Path;

fn main() {
	// Emplacement du dossier d'icônes
	let icons_dir = Path::new("assets/icons");
	
	// Créer le dossier des icônes s'il n'existe pas
	if !icons_dir.exists() {
		fs::create_dir_all(icons_dir).expect("Impossible de créer le dossier d'icônes");
		println!("Dossier d'icônes créé: {}", icons_dir.display());
	}
	
	// Vérifier la présence des icônes et afficher un message si elles sont manquantes
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
			println!("cargo:warning=L'icône {} est manquante", icon);
		}
	}
	
	// Indiquer au système de construction de surveiller le dossier d'icônes
	println!("cargo:rerun-if-changed=assets/icons");
}