use iced::{Background, Color};
use iced::widget::container;

/// Style du conteneur principal
pub struct ContainerStyle;

// Dans Iced 0.13, les styles utilisent des implémentations de From
impl From<ContainerStyle> for iced::widget::container::Appearance {
	fn from(_: ContainerStyle) -> Self {
		container::Appearance {
			background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
			text_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
			border_radius: 12.0,
			border_width: 1.0,
			border_color: Color::from_rgb(0.7, 0.7, 0.7),
		}
	}
}

/// Style pour les éléments du presse-papiers
pub struct ClipboardItemStyle {
	pub selected: bool,
}

impl From<ClipboardItemStyle> for container::Appearance {
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
		
		container::Appearance {
			background: Some(background),
			text_color: Some(text_color),
			border_radius: 8.0,
			border_width: 1.0,
			border_color: Color::from_rgb(0.8, 0.8, 0.8),
		}
	}
}

/// Style pour la barre d'outils
pub struct ToolbarStyle;

impl From<ToolbarStyle> for container::Appearance {
	fn from(_: ToolbarStyle) -> Self {
		container::Appearance {
			background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 1.0))),
			text_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
			border_radius: 8.0,
			border_width: 0.0,
			border_color: Color::TRANSPARENT,
		}
	}
}

/// Style pour la barre de recherche
pub struct SearchBarStyle;

impl From<SearchBarStyle> for container::Appearance {
	fn from(_: SearchBarStyle) -> Self {
		container::Appearance {
			background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
			text_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
			border_radius: 8.0,
			border_width: 1.0,
			border_color: Color::from_rgb(0.8, 0.8, 0.8),
		}
	}
}

/// Style pour les éléments épinglés
pub struct PinnedItemStyle;

impl From<PinnedItemStyle> for container::Appearance {
	fn from(_: PinnedItemStyle) -> Self {
		container::Appearance {
			background: Some(Background::Color(Color::from_rgb(1.0, 0.95, 0.9))),
			text_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
			border_radius: 8.0,
			border_width: 1.0,
			border_color: Color::from_rgb(1.0, 0.8, 0.5),
		}
	}
}