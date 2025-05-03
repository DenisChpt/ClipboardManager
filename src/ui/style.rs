use iced::{Background, Border, Color};
use iced::widget::container;

/// Style du conteneur principal
pub struct ContainerStyle;

// Dans Iced 0.13, les styles sont maintenant des fonctions qui prennent un thème
impl From<ContainerStyle> for container::Style {
	fn from(_: ContainerStyle) -> Self {
		container::Style {
			background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
			text_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
			border: Border {
				radius: 12.0.into(),
				width: 1.0,
				color: Color::from_rgb(0.7, 0.7, 0.7),
			},
			shadow: Default::default(),
		}
	}
}

/// Style pour les éléments du presse-papiers
pub struct ClipboardItemStyle {
	pub selected: bool,
}

impl From<ClipboardItemStyle> for container::Style {
	fn from(style: ClipboardItemStyle) -> Self {
		let (background, text_color) = if style.selected {
			(
				Background::Color(Color::from_rgb(0.9, 0.9, 1.0)),
				Color::from_rgb(0.2, 0.2, 0.8),
			)
		} else {
			(
				Background::Color(Color::from_rgb(1.0, 1.0, 1.0)),
				Color::from_rgb(0.2, 0.2, 0.2),
			)
		};
		
		container::Style {
			background: Some(background),
			text_color: Some(text_color),
			border: Border {
				radius: 8.0.into(),
				width: 1.0,
				color: Color::from_rgb(0.8, 0.8, 0.8),
			},
			shadow: Default::default(),
		}
	}
}

/// Style pour la barre d'outils
pub struct ToolbarStyle;

impl From<ToolbarStyle> for container::Style {
	fn from(_: ToolbarStyle) -> Self {
		container::Style {
			background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 1.0))),
			text_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
			border: Border {
				radius: 8.0.into(),
				width: 0.0,
				color: Color::TRANSPARENT,
			},
			shadow: Default::default(),
		}
	}
}

/// Style pour la barre de recherche
pub struct SearchBarStyle;

impl From<SearchBarStyle> for container::Style {
	fn from(_: SearchBarStyle) -> Self {
		container::Style {
			background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
			text_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
			border: Border {
				radius: 8.0.into(),
				width: 1.0,
				color: Color::from_rgb(0.8, 0.8, 0.8),
			},
			shadow: Default::default(),
		}
	}
}

/// Style pour les éléments épinglés
pub struct PinnedItemStyle;

impl From<PinnedItemStyle> for container::Style {
	fn from(_: PinnedItemStyle) -> Self {
		container::Style {
			background: Some(Background::Color(Color::from_rgb(1.0, 0.95, 0.9))),
			text_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
			border: Border {
				radius: 8.0.into(),
				width: 1.0,
				color: Color::from_rgb(1.0, 0.8, 0.5),
			},
			shadow: Default::default(),
		}
	}
}

// Fonctions de style corrigées avec 'static pour éviter les erreurs de durée de vie
pub fn container_style(_: &iced::Theme) -> container::Style {
	ContainerStyle.into()
}

pub fn clipboard_item_style(selected: bool) -> impl Fn(&iced::Theme) -> container::Style + 'static {
	move |_| {
		ClipboardItemStyle { selected }.into()
	}
}

pub fn toolbar_style(_: &iced::Theme) -> container::Style {
	ToolbarStyle.into()
}

pub fn search_bar_style(_: &iced::Theme) -> container::Style {
	SearchBarStyle.into()
}

pub fn pinned_item_style(_: &iced::Theme) -> container::Style {
	PinnedItemStyle.into()
}