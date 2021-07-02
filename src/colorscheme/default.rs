use crate::colorscheme::{ColorScheme, ColorHolder};
use tui::style::{Style, Modifier, Color};


pub fn new() -> ColorHolder {
    ColorHolder {
        main_bg: Color::Reset,
        main_fg: Color::Reset,
        menu_bg: Color::Reset,
        menu_fg: Color::Reset,
        memory_title_fg: Color::Reset,
        memory_border_fg: Color::Reset,
        memory_max_memory_text_fg: Color::Reset,
        memory_max_memory_dataset_fg: Color::Reset,
        memory_used_memory_text_fg: Color::Reset,
        memory_used_memory_dataset_fg: Color::Reset,
        cpu_title_fg: Color::Reset,
        cpu_border_fg: Color::Reset,
        cpu_sys_cpu_text_fg: Color::Reset,
        cpu_sys_cpu_dataset_fg: Color::Reset,
        cpu_user_cpu_text_fg: Color::Reset,
        cpu_user_cpu_dataset_fg: Color::Reset
    }
}