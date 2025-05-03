use crate::clipboard::{ClipboardContent, ClipboardItem};
use crate::config::Theme;
use crate::ui::Message;
use crate::ui::style::{toolbar_style, search_bar_style, pinned_item_style, clipboard_item_style};
use chrono::{DateTime, Utc};
use iced::widget::{button, column, container, horizontal_rule, image, row, text, text_input, Space};
use iced::{alignment, Length, Element};

/// Crée la barre d'outils
pub fn create_toolbar(current_theme: Theme) -> Element<'static, Message> {
	let title = text("Gestionnaire de presse-papiers")
		.size(18)
		.width(Length::Fill);

	let theme_button = match current_theme {
		Theme::Light => button("🌙").on_press(Message::SetTheme(Theme::Dark)),
		Theme::Dark => button("☀️").on_press(Message::SetTheme(Theme::Light)),
		Theme::System => {
			// Par défaut, utilisez une icône en fonction du thème système actuel
			if cfg!(target_os = "macos") && is_macos_dark_mode() {
				button("☀️").on_press(Message::SetTheme(Theme::Light))
			} else {
				button("🌙").on_press(Message::SetTheme(Theme::Dark))
			}
		}
	};

	let clear_button = button("🗑️")
		.on_press(Message::ClearItems)
		.width(Length::Shrink);

	// Correction: row! est un macro mais on doit toujours respecter l'API
	let toolbar = row![
		title,
		theme_button,
		Space::with_width(Length::Fixed(10.0)), // Remplacer horizontal_space
		clear_button
	]
	.padding(10)
	.spacing(10)
	.width(Length::Fill)
	.align_y(alignment::Vertical::Center); // Remplacer align_items

	container(toolbar)
		.style(toolbar_style)
		.width(Length::Fill)
		.into()
}

/// Crée la barre de recherche
pub fn create_search_bar(search_query: &str) -> Element<'static, Message> {
	let search_input = text_input("Rechercher...", search_query)
		.on_input(Message::SearchChanged)
		.padding(10)
		.width(Length::Fill);

	container(search_input)
		.padding(5)
		.style(search_bar_style)
		.width(Length::Fill)
		.into()
}

/// Crée un aperçu d'élément du presse-papiers
pub fn create_clipboard_item_view(item: &ClipboardItem) -> Element<'static, Message> {
	let item_id = item.id;
	let pinned = item.pinned;

	// Conteneur pour l'aperçu du contenu
	let content_preview: Element<'_, Message> = match &item.content {
		ClipboardContent::Text(text_val) => {
			let preview = if text_val.len() > 100 {
				format!("{}...", &text_val[..97])
			} else {
				text_val.clone()
			};
			
			// Utiliser directement la chaîne pour éviter les problèmes de durée de vie
			text::<iced::Theme, iced::Renderer>(preview).size(14).into()
		}
		ClipboardContent::Image(data, _metadata) => {
			// Créer un aperçu de l'image avec la nouvelle API
			let handle = image::Handle::from_bytes(data.clone());
			let img = image(handle)
				.width(Length::Fixed(100.0))
				.height(Length::Fixed(100.0))
				.content_fit(iced::ContentFit::Contain);
			
			container(img)
				.width(Length::Fill)
				.align_x(alignment::Horizontal::Center)
				.into()
		}
	};

	// Métadonnées (horodatage)
	let timestamp = format_timestamp(&item.timestamp);
	let metadata = text::<iced::Theme, iced::Renderer>(timestamp).size(12).color(iced::Color::from_rgb(0.5, 0.5, 0.5));

	// Boutons d'action
	let pin_icon = if pinned { "📌" } else { "📍" };
	
	let pin_button = button(text::<iced::Theme, iced::Renderer>(pin_icon))
		.on_press(Message::PinItem(item_id))
		.padding(5);
		
	let select_button = button("Utiliser")
		.on_press(Message::SelectItem(item_id))
		.padding(5);
		
	let remove_button = button("🗑️")
		.on_press(Message::RemoveItem(item_id))
		.padding(5);
	
	let buttons = row![
		pin_button,
		select_button,
		remove_button
	]
	.spacing(10);
	
	// Disposition de l'élément
	let content = column![
		content_preview,
		horizontal_rule(1),
		row![
			metadata,
			buttons
		]
		.width(Length::Fill)
		.spacing(10)
	]
	.spacing(10)
	.padding(10)
	.width(Length::Fill);
	
	// Conteneur principal de l'élément avec le style approprié
	if pinned {
		container(content)
			.style(pinned_item_style)
			.width(Length::Fill)
			.into()
	} else {
		container(content)
			.style(clipboard_item_style(false))
			.width(Length::Fill)
			.into()
	}
}

/// Formate un horodatage pour l'affichage
fn format_timestamp(timestamp: &DateTime<Utc>) -> String {
	let now = Utc::now();
	let diff = now.signed_duration_since(*timestamp);
	
	if diff.num_minutes() < 1 {
		return "À l'instant".to_string();
	} else if diff.num_minutes() < 60 {
		return format!("Il y a {} minutes", diff.num_minutes());
	} else if diff.num_hours() < 24 {
		return format!("Il y a {} heures", diff.num_hours());
	} else if diff.num_days() < 7 {
		return format!("Il y a {} jours", diff.num_days());
	} else {
		return timestamp.format("%d/%m/%Y %H:%M").to_string();
	}
}

/// Détecte si macOS est en mode sombre
#[cfg(target_os = "macos")]
fn is_macos_dark_mode() -> bool {
	use std::process::Command;
	
	let output = Command::new("defaults")
		.args(&["read", "-g", "AppleInterfaceStyle"])
		.output()
		.unwrap_or_else(|_| Default::default());
		
	String::from_utf8_lossy(&output.stdout).trim() == "Dark"
}

#[cfg(not(target_os = "macos"))]
fn is_macos_dark_mode() -> bool {
	false
}