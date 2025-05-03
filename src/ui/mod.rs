mod components;
mod style;
mod subscription;

use crate::clipboard::ClipboardItem;
use crate::config::Theme;
use components::{create_clipboard_item_view, create_search_bar, create_toolbar};
use iced::{Element, Subscription};
use iced::widget::{column, container, scrollable, text};
use style::container_style;
use uuid::Uuid;

/// État interne de l'interface
#[derive(Debug, Default, Clone)]
pub struct State {
	watcher_initialized: bool,
}

/// Messages UI
#[derive(Debug, Clone)]
pub enum Message {
	/// Éléments chargés depuis le stockage
	ItemsLoaded(Vec<ClipboardItem>),
	
	/// Nouvel élément détecté dans le presse-papiers
	NewClipboardItem(ClipboardItem),
	
	/// Sélectionner un élément pour le copier
	SelectItem(Uuid),
	
	/// Épingler/désépingler un élément
	PinItem(Uuid),
	
	/// Supprimer un élément
	RemoveItem(Uuid),
	
	/// Effacer tous les éléments non épinglés
	ClearItems,
	
	/// Changer le thème
	SetTheme(Theme),
	
	/// Mise à jour du terme de recherche
	SearchChanged(String),
	
	/// Recharger les éléments
	ReloadItems,
	
	/// Message vide (pour les actions qui ne nécessitent pas de mise à jour)
	None,
}

/// Abonnement aux événements du presse-papiers
pub fn clipboard_subscription() -> Subscription<Message> {
	subscription::clipboard_subscription()
}

/// Vue principale
pub fn view<'a>(
	_state: State,
	items: Vec<ClipboardItem>,
	search_query: String,
	theme: Theme,
) -> Element<'a, Message> {
	// Barre d'outils en haut
	let toolbar = create_toolbar(theme);
	
	// Barre de recherche
	let search_bar = create_search_bar(&search_query);
	
	// Liste des éléments
	let items_list = if items.is_empty() {
		column![
			container(text::<iced::Theme, iced::Renderer>("Aucun élément dans l'historique").size(16))
				.width(iced::Length::Fill)
				.align_x(iced::alignment::Horizontal::Center)
				.padding(20)
		].spacing(8).padding(8)
	} else {
		// Spécifions explicitement le type pour collect
		let elements: Vec<Element<'a, Message>> = items.iter()
			.map(|item| create_clipboard_item_view(item))
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