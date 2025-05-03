mod components;
mod style;
mod subscription;

use crate::clipboard::ClipboardItem;
use crate::config::Theme;
use components::{create_clipboard_item_view, create_search_bar, create_toolbar};
use iced::{Element, Subscription};
use iced::widget::{column, container, scrollable, text};
use style::ContainerStyle;
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
	_state: &'a State,
	items: &'a [ClipboardItem],
	search_query: &'a str,
	theme: Theme,
) -> Element<'a, Message> {
	// Barre d'outils en haut
	let toolbar = create_toolbar(theme);
	
	// Barre de recherche
	let search_bar = create_search_bar(search_query);
	
	// Liste des éléments
	let mut items_list = column().spacing(8).padding(8);
	
	if items.is_empty() {
		items_list = items_list.push(
			container(text("Aucun élément dans l'historique").size(16))
				.width(iced::Length::Fill)
				.align_x(iced::alignment::Horizontal::Center)
				.padding(20),
		);
	} else {
		for item in items {
			items_list = items_list.push(create_clipboard_item_view(item));
		}
	}
	
	// Conteneur scrollable pour la liste
	let scrollable_items = scrollable(items_list)
		.width(iced::Length::Fill)
		.height(iced::Length::Fill);
	
	// Mise en page principale
	let content = column()
		.push(toolbar)
		.push(search_bar)
		.push(scrollable_items)
		.spacing(10)
		.padding(10)
		.width(iced::Length::Fill)
		.height(iced::Length::Fill);
	
	// Conteneur principal avec coins arrondis
	container(content)
		.style(ContainerStyle)
		.width(iced::Length::Fill)
		.height(iced::Length::Fill)
		.into()
}