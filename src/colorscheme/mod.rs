pub mod nord;
pub mod default;

use tui::style::Style;

impl From<&str> for ColorScheme {
    fn from(s: &str) -> Self {
        match s {
            "nord" => nord::new(),
            _ => default::new()
        }
    }
}

pub struct ColorScheme {
    pub text: Style,
    pub menu_highlight_style: Style,
}