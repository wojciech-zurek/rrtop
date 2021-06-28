use crate::colorscheme::ColorScheme;
use tui::style::{Style, Modifier};


pub fn new() -> ColorScheme {
    ColorScheme {
        text: Style::default(),
        menu_highlight_style: Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED),
    }
}