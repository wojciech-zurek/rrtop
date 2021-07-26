use tui::style::Color;

use crate::colorscheme::ColorScheme;

pub fn new() -> ColorScheme {
    ColorScheme {
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
        memory_used_memory_text_fg: Color::Reset,
        memory_used_memory_sparkline_fg: Color::Reset,
        memory_used_memory_sparkline_baseline_fg: Color::Reset,
        memory_rss_memory_text_fg: Color::Reset,
        memory_rss_memory_sparkline_fg: Color::Reset,
        memory_rss_memory_sparkline_baseline_fg: Color::Reset,
        cpu_title_fg: Color::Reset,
        cpu_border_fg: Color::Reset,
        cpu_chart_line_fg: Color::Reset,
        cpu_chart_axis_fg: Color::Reset,
        cpu_sys_cpu_text_1_fg: Color::Reset,
        cpu_sys_cpu_text_2_fg: Color::Reset,
        cpu_sys_cpu_dataset_fg: Color::Reset,
        cpu_user_cpu_text_1_fg: Color::Reset,
        cpu_user_cpu_text_2_fg: Color::Reset,
        cpu_user_cpu_dataset_fg: Color::Reset,
        hit_rate_title_fg: Color::Reset,
        hit_rate_border_fg: Color::Reset,
        hit_rate_label_fg: Color::Reset,
        hit_rate_gauge_fg: Color::Reset,
        hit_rate_gauge_bg: Color::Reset,
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
        network_tx_sparkline_baseline_fg: Color::Reset,
        stat_title_fg: Color::Reset,
        stat_border_fg: Color::Reset,
        stat_table_header_fg: Color::Reset,
        stat_table_row_top_1_fg: Color::Reset,
        stat_table_row_top_2_fg: Color::Reset,
        stat_table_row_bottom_fg: Color::Reset,
        stat_table_row_highlight_bg: Color::Reset,
        calls_title_fg: Color::Reset,
        calls_border_fg: Color::Reset,
        calls_table_header_fg: Color::Reset,
        calls_table_row_gauge_fg: Color::Reset,
        calls_table_row_gauge_bg: Color::Reset,
        calls_table_row_top_1_fg: Color::Reset,
        calls_table_row_top_2_fg: Color::Reset,
        calls_table_row_bottom_fg: Color::Reset,
        calls_table_row_highlight_bg: Color::Reset,
        warning_title_fg: Color::Reset,
        warning_border_fg: Color::Reset,
        warning_text_1_fg: Color::Reset,
        warning_text_2_fg: Color::Reset,
    }
}