use iced::{Background, Border, Color};
use iced::widget::container;
use iced::widget::button;

/// Style du conteneur principal
pub struct ContainerStyle {
	pub dark_mode: bool,
}

impl From<ContainerStyle> for container::Style {
	fn from(style: ContainerStyle) -> Self {
		if style.dark_mode {
			container::Style {
				background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
				text_color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
				border: Border {
					radius: 12.0.into(),
					width: 1.0,
					color: Color::from_rgb(0.3, 0.3, 0.3),
				},
				shadow: Default::default(),
			}
		} else {
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
}

/// Style pour les éléments du presse-papiers - exporter publiquement
#[derive(Clone, Copy)]
pub struct ClipboardItemStyle {
	pub selected: bool,
	pub dark_mode: bool,
}

impl From<ClipboardItemStyle> for container::Style {
	fn from(style: ClipboardItemStyle) -> Self {
		let (background, text_color) = if style.dark_mode {
			if style.selected {
				(
					Background::Color(Color::from_rgb(0.25, 0.25, 0.35)), // sélectionné en mode sombre
					Color::from_rgb(0.9, 0.9, 1.0),
				)
			} else {
				(
					Background::Color(Color::from_rgb(0.2, 0.2, 0.2)), // normal en mode sombre
					Color::from_rgb(0.8, 0.8, 0.8),
				)
			}
		} else {
			if style.selected {
				(
					Background::Color(Color::from_rgb(0.9, 0.9, 1.0)),
					Color::from_rgb(0.2, 0.2, 0.8),
				)
			} else {
				(
					Background::Color(Color::from_rgb(1.0, 1.0, 1.0)),
					Color::from_rgb(0.2, 0.2, 0.2),
				)
			}
		};
		
		container::Style {
			background: Some(background),
			text_color: Some(text_color),
			border: Border {
				radius: 8.0.into(),
				width: 1.0,
				color: if style.dark_mode {
					Color::from_rgb(0.4, 0.4, 0.4)
				} else {
					Color::from_rgb(0.8, 0.8, 0.8)
				},
			},
			shadow: Default::default(),
		}
	}
}

/// Style pour la barre d'outils
pub struct ToolbarStyle {
	pub dark_mode: bool,
}

impl From<ToolbarStyle> for container::Style {
	fn from(style: ToolbarStyle) -> Self {
		container::Style {
			background: Some(Background::Color(if style.dark_mode {
				Color::from_rgb(0.18, 0.18, 0.2)
			} else {
				Color::from_rgb(0.95, 0.95, 1.0)
			})),
			text_color: Some(if style.dark_mode {
				Color::from_rgb(0.9, 0.9, 0.9)
			} else {
				Color::from_rgb(0.2, 0.2, 0.2)
			}),
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
pub struct SearchBarStyle {
	pub dark_mode: bool,
}

impl From<SearchBarStyle> for container::Style {
	fn from(style: SearchBarStyle) -> Self {
		container::Style {
			background: Some(Background::Color(if style.dark_mode {
				Color::from_rgb(0.25, 0.25, 0.25)
			} else {
				Color::from_rgb(1.0, 1.0, 1.0)
			})),
			text_color: Some(if style.dark_mode {
				Color::from_rgb(0.9, 0.9, 0.9)
			} else {
				Color::from_rgb(0.2, 0.2, 0.2)
			}),
			border: Border {
				radius: 8.0.into(),
				width: 1.0,
				color: if style.dark_mode {
					Color::from_rgb(0.4, 0.4, 0.4)
				} else {
					Color::from_rgb(0.8, 0.8, 0.8)
				},
			},
			shadow: Default::default(),
		}
	}
}

/// Style pour les éléments épinglés
pub struct PinnedItemStyle {
	pub dark_mode: bool,
}

impl From<PinnedItemStyle> for container::Style {
	fn from(style: PinnedItemStyle) -> Self {
		container::Style {
			background: Some(Background::Color(if style.dark_mode {
				Color::from_rgb(0.35, 0.25, 0.2)
			} else {
				Color::from_rgb(1.0, 0.95, 0.9)
			})),
			text_color: Some(if style.dark_mode {
				Color::from_rgb(1.0, 0.8, 0.7)
			} else {
				Color::from_rgb(0.2, 0.2, 0.2)
			}),
			border: Border {
				radius: 8.0.into(),
				width: 1.0,
				color: if style.dark_mode {
					Color::from_rgb(0.7, 0.5, 0.3)
				} else {
					Color::from_rgb(1.0, 0.8, 0.5)
				},
			},
			shadow: Default::default(),
		}
	}
}

/// Style de bouton premium avec coins arrondis
pub struct RoundButtonStyle {
	pub dark_mode: bool,
}

impl From<RoundButtonStyle> for button::Style {
	fn from(style: RoundButtonStyle) -> Self {
		button::Style {
			background: Some(Background::Color(if style.dark_mode {
				Color::from_rgb(0.3, 0.3, 0.35)
			} else {
				Color::from_rgb(0.9, 0.9, 0.95)
			})),
			text_color: if style.dark_mode {
				Color::from_rgb(0.9, 0.9, 0.9)
			} else {
				Color::from_rgb(0.2, 0.2, 0.2)
			},
			border: Border {
				radius: 18.0.into(), // Plus arrondi pour un look premium
				width: 1.0,
				color: if style.dark_mode {
					Color::from_rgb(0.5, 0.5, 0.5)
				} else {
					Color::from_rgb(0.7, 0.7, 0.7)
				},
			},
			shadow: Default::default(),
		}
	}
}

// Fonctions de style mises à jour
pub fn container_style(theme: &iced::Theme) -> container::Style {
	let is_dark = match theme {
		iced::Theme::Dark => true,
		_ => false,
	};
	ContainerStyle { dark_mode: is_dark }.into()
}

pub fn clipboard_item_style(selected: bool, theme: &iced::Theme) -> container::Style {
	let is_dark = match theme {
		iced::Theme::Dark => true,
		_ => false,
	};
	ClipboardItemStyle { selected, dark_mode: is_dark }.into()
}

pub fn toolbar_style(theme: &iced::Theme) -> container::Style {
	let is_dark = match theme {
		iced::Theme::Dark => true,
		_ => false,
	};
	ToolbarStyle { dark_mode: is_dark }.into()
}

pub fn search_bar_style(theme: &iced::Theme) -> container::Style {
	let is_dark = match theme {
		iced::Theme::Dark => true,
		_ => false,
	};
	SearchBarStyle { dark_mode: is_dark }.into()
}

pub fn pinned_item_style(theme: &iced::Theme) -> container::Style {
	let is_dark = match theme {
		iced::Theme::Dark => true,
		_ => false,
	};
	PinnedItemStyle { dark_mode: is_dark }.into()
}

pub fn round_button_style(theme: &iced::Theme) -> button::Style {
	let is_dark = match theme {
		iced::Theme::Dark => true,
		_ => false,
	};
	RoundButtonStyle { dark_mode: is_dark }.into()
}