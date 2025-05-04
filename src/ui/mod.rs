mod components;
mod style;
mod subscription;

use crate::clipboard::ClipboardItem;
use crate::config::Theme;
use components::{create_clipboard_item_view, create_search_bar, create_toolbar};
use iced::{Element, Subscription, keyboard};
use iced::widget::{column, container, scrollable, text};
use style::container_style;
use uuid::Uuid;

/// État interne de l'interface
#[derive(Debug, Default, Clone)]
pub struct State {
	pub selected_index: usize,
}

/// Messages UI
#[derive(Debug, Clone)]
pub enum Message {
	ItemsLoaded(Vec<ClipboardItem>),
	NewClipboardItem(ClipboardItem),
	UseItem(Uuid),
	PinItem(Uuid),
	RemoveItem(Uuid),
	ClearItems,
	SetTheme(Theme),
	SearchChanged(String),
	ReloadItems,
	NavigateUp,
	NavigateDown,
	UseSelected,
	None,
}

/// Abonnement aux événements du presse-papiers
pub fn clipboard_subscription() -> Subscription<Message> {
	subscription::clipboard_subscription()
}

/// Abonnement aux événements clavier
pub fn keyboard_subscription() -> Subscription<Message> {
	keyboard::on_key_press(|key, _modifiers| {
		match key {
			keyboard::Key::Named(keyboard::key::Named::ArrowUp) => Some(Message::NavigateUp),
			keyboard::Key::Named(keyboard::key::Named::ArrowDown) => Some(Message::NavigateDown),
			keyboard::Key::Named(keyboard::key::Named::Enter) => Some(Message::UseSelected),
			_ => None,
		}
	})
}

/// Vue principale
pub fn view<'a>(
	state: State,
	items: Vec<ClipboardItem>,
	search_query: String,
	theme: Theme,
	_iced_theme: iced::Theme,
) -> Element<'a, Message> {
	// Barre d'outils en haut
	let toolbar = create_toolbar(theme, &iced::Theme::Light);
	
	// Barre de recherche
	let search_bar = create_search_bar(&search_query, &iced::Theme::Light);
	
	// Liste des éléments
	let items_list = if items.is_empty() {
		column![
			container(text("Aucun élément dans l'historique").size(16))
				.width(iced::Length::Fill)
				.align_x(iced::alignment::Horizontal::Center)
				.padding(20)
		].spacing(8).padding(8)
	} else {
		let elements: Vec<Element<'a, Message>> = items.iter()
			.enumerate()
			.map(|(index, item)| create_clipboard_item_view(item, index == state.selected_index, &iced::Theme::Light))
			.collect();
		column(elements).spacing(8).padding(8)
	};
	
	// Conteneur scrollable pour la liste
	let scrollable_items = scrollable(items_list)
		.width(iced::Length::Fill)
		.height(iced::Length::Fill);
	
	// Mise en page principale
	let content = column![
		toolbar,
		search_bar,
		scrollable_items
	]
	.spacing(10)
	.padding(10)
	.width(iced::Length::Fill)
	.height(iced::Length::Fill);
	
	// Conteneur principal avec coins arrondis
	container(content)
		.style(container_style)
		.width(iced::Length::Fill)
		.height(iced::Length::Fill)
		.into()
}
