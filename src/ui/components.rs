use crate::clipboard::{ClipboardContent, ClipboardItem};
use crate::config::Theme;
use crate::ui::Message;
use crate::ui::style::{ClipboardItemStyle, PinnedItemStyle, SearchBarStyle, ToolbarStyle};
use chrono::{DateTime, Utc};
use iced::widget::{button, column, container, horizontal_rule, horizontal_space, image, row, text, text_input};
use iced::{alignment, Color, Element, Length};

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

	let toolbar = row()
		.push(title)
		.push(theme_button)
		.push(horizontal_space(Length::Units(10)))
		.push(clear_button)
		.padding(10)
		.spacing(5)
		.width(Length::Fill)
		.align_items(alignment::Alignment::Center);

	container(toolbar)
		.style(ToolbarStyle)
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
		.style(SearchBarStyle)
		.width(Length::Fill)
		.into()
}

/// Crée un aperçu d'élément du presse-papiers
pub fn create_clipboard_item_view(item: &ClipboardItem) -> Element<'static, Message> {
	let item_id = item.id;
	let pinned = item.pinned;

	// Conteneur pour l'aperçu du contenu
	let content_preview = match &item.content {
		ClipboardContent::Text(text) => {
			let preview = if text.len() > 100 {
				format!("{}...", &text[..97])
			} else {
				text.clone()
			};
			
			text(&preview).size(14).into()
		}
		ClipboardContent::Image(data, metadata) => {
			// Créer un aperçu de l'image
			let handle = image::Handle::from_memory(data.clone());
			let img = image(handle)
				.width(Length::Units(100))
				.height(Length::Units(100))
				.content_fit(iced::ContentFit::Contain);
			
			container(img)
				.width(Length::Fill)
				.align_x(alignment::Horizontal::Center)
				.into()
		}
	};

	// Métadonnées (horodatage)
	let timestamp = format_timestamp(&item.timestamp);
	let metadata = text(timestamp).size(12).color(Color::from_rgb(0.5, 0.5, 0.5));

	// Boutons d'action
	let pin_icon = if pinned { "📌" } else { "📍" };
	
	let pin_button = button(text(pin_icon))
		.on_press(Message::PinItem(item_id))
		.padding(5);
		
	let select_button = button("Utiliser")
		.on_press(Message::SelectItem(item_id))
		.padding(5);
		
	let remove_button = button("🗑️")
		.on_press(Message::RemoveItem(item_id))
		.padding(5);
	
	let buttons = row()
		.push(pin_button)
		.push(horizontal_space(Length::Units(10)))
		.push(select_button)
		.push(horizontal_space(Length::Units(10)))
		.push(remove_button)
		.spacing(5);
	
	// Disposition de l'élément
	let content = column()
		.push(content_preview)
		.push(horizontal_rule(1))
		.push(
			row()
				.push(metadata)
				.push(horizontal_space(Length::Fill))
				.push(buttons)
				.width(Length::Fill)
		)
		.spacing(10)
		.padding(10)
		.width(Length::Fill);
	
	// Conteneur principal de l'élément avec le style approprié
	if pinned {
		container(content)
			.style(PinnedItemStyle)
			.width(Length::Fill)
			.into()
	} else {
		container(content)
			.style(ClipboardItemStyle { selected: false })
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