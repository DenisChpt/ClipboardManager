use crate::clipboard::{ClipboardContent, ClipboardItem};
use crate::config::Theme;
use crate::ui::Message;
use crate::ui::style::{toolbar_style, search_bar_style, pinned_item_style, clipboard_item_style};
use chrono::{DateTime, Utc};
use iced::widget::{button, column, container, horizontal_rule, image, row, text, text_input, Space, svg};
use iced::{alignment, Length, Element};

/// Crée la barre d'outils
pub fn create_toolbar(current_theme: Theme) -> Element<'static, Message> {
	let title = text("Gestionnaire de presse-papiers")
		.size(18)
		.width(Length::Fill);

	let theme_button = match current_theme {
		Theme::Light => {
			let moon_icon = svg::Handle::from_path("assets/icons/moon.svg");
			button(svg(moon_icon).width(Length::Fixed(20.0)).height(Length::Fixed(20.0)))
				.on_press(Message::SetTheme(Theme::Dark))
		},
		Theme::Dark => {
			let sun_icon = svg::Handle::from_path("assets/icons/sun.svg");
			button(svg(sun_icon).width(Length::Fixed(20.0)).height(Length::Fixed(20.0)))
				.on_press(Message::SetTheme(Theme::Light))
		},
		Theme::System => {
			if cfg!(target_os = "macos") && is_macos_dark_mode() {
				let sun_icon = svg::Handle::from_path("assets/icons/sun.svg");
				button(svg(sun_icon).width(Length::Fixed(20.0)).height(Length::Fixed(20.0)))
					.on_press(Message::SetTheme(Theme::Light))
			} else {
				let moon_icon = svg::Handle::from_path("assets/icons/moon.svg");
				button(svg(moon_icon).width(Length::Fixed(20.0)).height(Length::Fixed(20.0)))
					.on_press(Message::SetTheme(Theme::Dark))
			}
		}
	};

	let trash_icon = svg::Handle::from_path("assets/icons/trash.svg");
	let clear_button = button(svg(trash_icon).width(Length::Fixed(20.0)).height(Length::Fixed(20.0)))
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
	// Utiliser des icônes SVG au lieu d'emojis
	let pin_icon = if pinned {
		svg::Handle::from_path("assets/icons/pinned.svg")
	} else {
		svg::Handle::from_path("assets/icons/pin.svg")
	};
	
	let pin_button = button(svg(pin_icon).width(Length::Fixed(20.0)).height(Length::Fixed(20.0)))
		.on_press(Message::PinItem(item_id))
		.padding(5);
	
	let use_icon = svg::Handle::from_path("assets/icons/use.svg");
	let select_button = button(
		row![
			svg(use_icon).width(Length::Fixed(20.0)).height(Length::Fixed(20.0)),
			text("Utiliser").size(14)
		].spacing(5)
	)
	.on_press(Message::SelectItem(item_id))
	.padding(5);
	
	let trash_icon = svg::Handle::from_path("assets/icons/trash.svg");
	let remove_button = button(svg(trash_icon).width(Length::Fixed(20.0)).height(Length::Fixed(20.0)))
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