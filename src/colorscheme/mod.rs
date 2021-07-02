pub mod nord;
pub mod default;

use tui::style::{Style, Color, Modifier};

pub struct ColorHolder {
    main_bg: Color,
    main_fg: Color,
    menu_bg: Color,
    menu_fg: Color,

    memory_title_fg: Color,
    memory_border_fg: Color,
    memory_max_memory_text_fg: Color,
    memory_max_memory_dataset_fg: Color,
    memory_used_memory_text_fg: Color,
    memory_used_memory_dataset_fg: Color,
    cpu_title_fg: Color,
    cpu_border_fg: Color,
    cpu_sys_cpu_text_fg: Color,
    cpu_sys_cpu_dataset_fg: Color,
    cpu_user_cpu_text_fg: Color,
    cpu_user_cpu_dataset_fg: Color,
}

impl From<&str> for ColorHolder {
    fn from(s: &str) -> Self {
        match s {
            "nord" => nord::new(),
            _ => default::new()
        }
    }
}

pub struct ColorScheme {
    pub main: Style,
    pub menu_highlight: Style,

    pub memory_title: Style,
    pub memory_border: Style,
    pub memory_chart: Style,
    pub memory_max_memory_text: Style,
    pub memory_max_memory_dataset: Style,
    pub memory_used_memory_text: Style,
    pub memory_used_memory_dataset: Style,

    pub cpu_title: Style,
    pub cpu_border: Style,
    pub cpu_chart: Style,
    pub cpu_sys_cpu_text: Style,
    pub cpu_sys_cpu_dataset: Style,
    pub cpu_user_cpu_text: Style,
    pub cpu_user_cpu_dataset: Style,
}

impl ColorScheme {
    pub fn new(ch: ColorHolder) -> ColorScheme {
        ColorScheme {
            main: Style::default().bg(ch.main_bg).fg(ch.main_fg),
            menu_highlight: Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(ch.menu_fg)
                .bg(ch.menu_bg),

            memory_title: Style::default().fg(ch.memory_title_fg),
            memory_border: Style::default().fg(ch.memory_border_fg),
            memory_chart: Style::default().bg(ch.main_bg),
            memory_max_memory_text: Style::default().fg(ch.memory_max_memory_text_fg).add_modifier(Modifier::BOLD),
            memory_max_memory_dataset: Style::default().fg(ch.memory_max_memory_dataset_fg),
            memory_used_memory_text: Style::default().fg(ch.memory_used_memory_text_fg).add_modifier(Modifier::BOLD),
            memory_used_memory_dataset: Style::default().fg(ch.memory_used_memory_dataset_fg),

            cpu_title: Style::default().fg(ch.cpu_title_fg),
            cpu_border: Style::default().fg(ch.cpu_border_fg).bg(ch.main_bg),
            cpu_chart: Style::default().bg(ch.main_bg),
            cpu_sys_cpu_text: Style::default().fg(ch.cpu_sys_cpu_text_fg).add_modifier(Modifier::BOLD),
            cpu_sys_cpu_dataset: Style::default().fg(ch.cpu_sys_cpu_dataset_fg),
            cpu_user_cpu_text: Style::default().fg(ch.cpu_user_cpu_text_fg).add_modifier(Modifier::BOLD),
            cpu_user_cpu_dataset: Style::default().fg(ch.cpu_user_cpu_dataset_fg),
        }
    }
}

impl From<&str> for ColorScheme {
    fn from(s: &str) -> Self {
        let ch = ColorHolder::from(s);
        ColorScheme::new(ch)
    }
}