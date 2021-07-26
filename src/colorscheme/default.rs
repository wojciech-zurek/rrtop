use tui::style::Color;
use tui::style::Color::{Black, Blue, Cyan, DarkGray, Gray, Green, Red};

use crate::colorscheme::ColorScheme;

pub fn new() -> ColorScheme {
    ColorScheme {
        main_bg: Black,
        main_fg: Color::Reset,

        menu_bg: Black,
        menu_fg: DarkGray,
        menu_divider_fg: DarkGray,
        menu_highlight_bg: Black,
        menu_highlight_fg: Color::Reset,
        status_bar_fg: DarkGray,

        memory_title_fg: Color::Reset,
        memory_border_fg: DarkGray,
        memory_max_memory_text_fg: Blue,
        memory_used_memory_text_fg: Blue,
        memory_used_memory_sparkline_fg: Blue,
        memory_used_memory_sparkline_baseline_fg: Black,
        memory_rss_memory_text_fg: Green,
        memory_rss_memory_sparkline_fg: Green,
        memory_rss_memory_sparkline_baseline_fg: DarkGray,

        cpu_title_fg: Color::Reset,
        cpu_border_fg: DarkGray,
        cpu_chart_line_fg: DarkGray,
        cpu_chart_axis_fg: DarkGray,
        cpu_sys_cpu_text_1_fg: Color::Reset,
        cpu_sys_cpu_text_2_fg: Red,
        cpu_sys_cpu_dataset_fg: Red,
        cpu_user_cpu_text_1_fg: Color::Reset,
        cpu_user_cpu_text_2_fg: Green,
        cpu_user_cpu_dataset_fg: Green,

        hit_rate_title_fg: Color::Reset,
        hit_rate_border_fg: DarkGray,
        hit_rate_label_fg: Blue,
        hit_rate_gauge_fg: Blue,
        hit_rate_gauge_bg: DarkGray,

        throughput_title_fg: Color::Reset,
        throughput_border_fg: DarkGray,
        throughput_total_commands_text_fg: Blue,
        throughput_ops_text_fg: Blue,
        throughput_sparkline_fg: Blue,
        throughput_sparkline_baseline_fg: DarkGray,

        network_title_fg: Color::Reset,
        network_border_fg: DarkGray,
        network_rx_total_text_fg: Red,
        network_rx_s_text_fg: Red,
        network_rx_sparkline_fg: Red,
        network_rx_sparkline_baseline_fg: DarkGray,
        network_tx_total_text_fg: Blue,
        network_tx_s_text_fg: Blue,
        network_tx_sparkline_fg: Blue,
        network_tx_sparkline_baseline_fg: DarkGray,

        stat_title_fg: Color::Reset,
        stat_border_fg: DarkGray,
        stat_table_header_fg: Cyan,
        stat_table_row_gauge_fg: Blue,
        stat_table_row_gauge_bg: Gray,
        stat_table_row_top_1_fg: Color::Reset,
        stat_table_row_top_2_fg: Blue,
        stat_table_row_bottom_fg: Gray,
        stat_table_row_highlight_bg: DarkGray,

        calls_title_fg: Color::Reset,
        calls_border_fg: DarkGray,
        calls_table_header_fg: Cyan,
        calls_table_row_gauge_fg: Blue,
        calls_table_row_gauge_bg: Gray,
        calls_table_row_top_1_fg: Color::Reset,
        calls_table_row_top_2_fg: Blue,
        calls_table_row_bottom_fg: Gray,
        calls_table_row_highlight_bg: DarkGray,

        warning_title_fg: Color::Reset,
        warning_border_fg: DarkGray,
        warning_text_1_fg: Red,
        warning_text_2_fg: Green,
    }
}