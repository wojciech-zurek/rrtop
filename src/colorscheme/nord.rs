use crate::colorscheme::{ColorScheme, ColorHolder};
use tui::style::{Style, Modifier, Color};

pub fn new() -> ColorHolder {
    ColorHolder {
        main_bg: Color::Rgb(46, 52, 64),
        main_fg: Color::Rgb(216, 222, 233),
        menu_bg: Color::Reset,
        menu_fg: Color::Rgb(129, 161, 193),

        memory_title_fg: Color::Rgb(143, 188, 187),
        memory_border_fg: Color::Rgb(76, 86, 106),
        memory_max_memory_text_fg: Color::Rgb(191, 97, 106),
        memory_max_memory_dataset_fg: Color::Rgb(191, 97, 106),
        memory_used_memory_text_fg: Color::Rgb(129, 161, 193),
        memory_used_memory_dataset_fg: Color::Rgb(129, 161, 193),

        cpu_title_fg:  Color::Rgb(143, 188, 187),
        cpu_border_fg: Color::Rgb(76, 86, 106),
        cpu_sys_cpu_text_fg: Color::Rgb(191, 97, 106),
        cpu_sys_cpu_dataset_fg: Color::Rgb(191, 97, 106),
        cpu_user_cpu_text_fg: Color::Rgb(129, 161, 193),
        cpu_user_cpu_dataset_fg: Color::Rgb(129, 161, 193),
    }
}