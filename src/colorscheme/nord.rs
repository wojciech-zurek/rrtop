use crate::colorscheme::{ColorScheme};
use tui::style::{Style, Modifier, Color};

pub const NORD_0: Color = Color::Rgb(46, 52, 64);
pub const NORD_1: Color = Color::Rgb(59, 66, 82);
pub const NORD_2: Color = Color::Rgb(67, 76, 94);
pub const NORD_3: Color = Color::Rgb(76, 86, 106);
pub const NORD_4: Color = Color::Rgb(216, 222, 233);
pub const NORD_5: Color = Color::Rgb(229, 233, 240);
pub const NORD_6: Color = Color::Rgb(236, 239, 244);
pub const NORD_7: Color = Color::Rgb(143, 188, 187);//green
pub const NORD_8: Color = Color::Rgb(136, 192, 208);//more blue
pub const NORD_9: Color = Color::Rgb(129, 161, 193);//dark blue
pub const NORD_10: Color = Color::Rgb(94, 129, 172);//more dark blue
pub const NORD_11: Color = Color::Rgb(191, 97, 106);//red
pub const NORD_12: Color = Color::Rgb(208, 135, 112);//orange
pub const NORD_13: Color = Color::Rgb(235, 203, 139);//yellow
pub const NORD_14: Color = Color::Rgb(163, 190, 140);//green
pub const NORD_15: Color = Color::Rgb(180, 142, 173);//pu

pub fn new() -> ColorScheme {
    ColorScheme {
        text: Style::default(),
        menu_highlight_style: Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED)
            .fg(Color::Rgb(129, 161, 193))
    }
}