pub mod nord;
pub mod default;

use tui::style::{Style, Color, Modifier};

pub struct ColorHolder {
    main_bg: Color,
    main_fg: Color,

    menu_bg: Color,
    menu_fg: Color,
    menu_divider_fg: Color,
    menu_highlight_bg: Color,
    menu_highlight_fg: Color,

    status_bar_fg: Color,

    memory_title_fg: Color,
    memory_border_fg: Color,
    memory_max_memory_text_fg: Color,
    memory_max_memory_dataset_fg: Color,
    memory_used_memory_text_fg: Color,
    memory_used_memory_dataset_fg: Color,

    cpu_title_fg: Color,
    cpu_border_fg: Color,
    cpu_chart_line_fg: Color,
    cpu_sys_cpu_text_fg: Color,
    cpu_sys_cpu_dataset_fg: Color,
    cpu_user_cpu_text_fg: Color,
    cpu_user_cpu_dataset_fg: Color,

    throughput_title_fg: Color,
    throughput_border_fg: Color,
    throughput_total_commands_text_fg: Color,
    throughput_ops_text_fg: Color,
    throughput_sparkline_fg: Color,
    throughput_sparkline_baseline_fg: Color,

    network_title_fg: Color,
    network_border_fg: Color,
    network_rx_total_text_fg: Color,
    network_rx_s_text_fg: Color,
    network_rx_sparkline_fg: Color,
    network_rx_sparkline_baseline_fg: Color,
    network_tx_total_text_fg: Color,
    network_tx_s_text_fg: Color,
    network_tx_sparkline_fg: Color,
    network_tx_sparkline_baseline_fg: Color,
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

    pub menu: Style,
    pub menu_divider: Style,
    pub menu_highlight: Style,

    pub status_bar: Style,

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
    pub cpu_chart_line: Style,
    pub cpu_sys_cpu_text: Style,
    pub cpu_sys_cpu_dataset: Style,
    pub cpu_user_cpu_text: Style,
    pub cpu_user_cpu_dataset: Style,

    pub throughput_title: Style,
    pub throughput_border: Style,
    pub throughput_sparkline: Style,
    pub throughput_sparkline_baseline: Style,
    pub throughput_total_commands_text: Style,
    pub throughput_ops_text: Style,

    pub network_title: Style,
    pub network_border: Style,
    pub network_rx_total_text: Style,
    pub network_rx_s_text: Style,
    pub network_rx_sparkline: Style,
    pub network_rx_sparkline_baseline: Style,
    pub network_tx_total_text: Style,
    pub network_tx_s_text: Style,
    pub network_tx_sparkline: Style,
    pub network_tx_sparkline_baseline: Style,

}

impl ColorScheme {
    pub fn new(ch: ColorHolder) -> ColorScheme {
        ColorScheme {
            main: Style::default().fg(ch.main_fg).bg(ch.main_bg),

            menu: Style::default().fg(ch.menu_fg).bg(ch.menu_bg),
            menu_divider: Style::default().fg(ch.menu_divider_fg),
            menu_highlight: Style::default()
                .fg(ch.menu_highlight_fg)
                .bg(ch.menu_highlight_bg)
                .add_modifier(Modifier::BOLD),

            status_bar: Style::default().fg(ch.status_bar_fg),

            memory_title: Style::default().fg(ch.memory_title_fg),
            memory_border: Style::default().fg(ch.memory_border_fg),
            memory_chart: Style::default().bg(ch.main_bg),
            memory_max_memory_text: Style::default()
                .fg(ch.memory_max_memory_text_fg)
                .add_modifier(Modifier::BOLD),
            memory_max_memory_dataset: Style::default().fg(ch.memory_max_memory_dataset_fg),
            memory_used_memory_text: Style::default()
                .fg(ch.memory_used_memory_text_fg)
                .add_modifier(Modifier::BOLD),
            memory_used_memory_dataset: Style::default().fg(ch.memory_used_memory_dataset_fg),

            cpu_title: Style::default().fg(ch.cpu_title_fg),
            cpu_border: Style::default().fg(ch.cpu_border_fg),
            cpu_chart: Style::default().bg(ch.main_bg),
            cpu_chart_line: Style::default().fg(ch.cpu_chart_line_fg),
            cpu_sys_cpu_text: Style::default()
                .fg(ch.cpu_sys_cpu_text_fg)
                .add_modifier(Modifier::BOLD),
            cpu_sys_cpu_dataset: Style::default().fg(ch.cpu_sys_cpu_dataset_fg),
            cpu_user_cpu_text: Style::default()
                .fg(ch.cpu_user_cpu_text_fg)
                .add_modifier(Modifier::BOLD),
            cpu_user_cpu_dataset: Style::default().fg(ch.cpu_user_cpu_dataset_fg),

            throughput_title: Style::default().fg(ch.throughput_title_fg),
            throughput_border: Style::default().fg(ch.throughput_border_fg),
            throughput_sparkline: Style::default().fg(ch.throughput_sparkline_fg),
            throughput_sparkline_baseline: Style::default().fg(ch.throughput_sparkline_baseline_fg),
            throughput_total_commands_text: Style::default()
                .fg(ch.throughput_total_commands_text_fg)
                .add_modifier(Modifier::BOLD),
            throughput_ops_text: Style::default()
                .fg(ch.throughput_ops_text_fg)
                .add_modifier(Modifier::BOLD),

            network_title: Style::default().fg(ch.network_title_fg),
            network_border: Style::default().fg(ch.network_border_fg),
            network_rx_total_text: Style::default()
                .fg(ch.network_rx_total_text_fg)
                .add_modifier(Modifier::BOLD),
            network_rx_s_text: Style::default()
                .fg(ch.network_rx_s_text_fg)
                .add_modifier(Modifier::BOLD),
            network_rx_sparkline: Style::default().fg(ch.network_rx_sparkline_fg),
            network_rx_sparkline_baseline: Style::default().fg(ch.network_rx_sparkline_baseline_fg),
            network_tx_total_text: Style::default()
                .fg(ch.network_tx_total_text_fg)
                .add_modifier(Modifier::BOLD),
            network_tx_s_text: Style::default()
                .fg(ch.network_tx_s_text_fg)
                .add_modifier(Modifier::BOLD),
            network_tx_sparkline: Style::default().fg(ch.network_tx_sparkline_fg),
            network_tx_sparkline_baseline: Style::default().fg(ch.network_tx_sparkline_baseline_fg),
        }
    }
}

impl From<&str> for ColorScheme {
    fn from(s: &str) -> Self {
        let ch = ColorHolder::from(s);
        ColorScheme::new(ch)
    }
}