use crate::colorscheme::{ColorScheme, ColorHolder};
use tui::style::{Style, Modifier, Color};


pub fn new() -> ColorHolder {
    ColorHolder {
        main_bg: Color::Reset,
        main_fg: Color::Reset,
        menu_bg: Color::Reset,
        menu_fg: Color::Reset,
        menu_divider_fg: Color::Reset,
        menu_highlight_bg: Color::Reset,
        menu_highlight_fg: Color::Reset,
        status_bar_fg: Color::Reset,
        memory_title_fg: Color::Reset,
        memory_border_fg: Color::Reset,
        memory_max_memory_text_fg: Color::Reset,
        memory_max_memory_dataset_fg: Color::Reset,
        memory_used_memory_text_fg: Color::Reset,
        memory_used_memory_dataset_fg: Color::Reset,
        cpu_title_fg: Color::Reset,
        cpu_border_fg: Color::Reset,
        cpu_chart_line_fg: Color::Reset,
        cpu_sys_cpu_text_fg: Color::Reset,
        cpu_sys_cpu_dataset_fg: Color::Reset,
        cpu_user_cpu_text_fg: Color::Reset,
        cpu_user_cpu_dataset_fg: Color::Reset,
        throughput_title_fg: Color::Reset,
        throughput_border_fg: Color::Reset,
        throughput_total_commands_text_fg: Color::Reset,
        throughput_ops_text_fg: Color::Reset,
        throughput_sparkline_fg: Color::Reset,
        throughput_sparkline_baseline_fg: Color::Reset,
        network_title_fg: Color::Reset,
        network_border_fg: Color::Reset,
        network_rx_total_text_fg: Color::Reset,
        network_rx_s_text_fg: Color::Reset,
        network_rx_sparkline_fg: Color::Reset,
        network_rx_sparkline_baseline_fg: Color::Reset,
        network_tx_total_text_fg: Color::Reset,
        network_tx_s_text_fg: Color::Reset,
        network_tx_sparkline_fg: Color::Reset,
        network_tx_sparkline_baseline_fg: Color::Reset
    }
}