use tui::style::Color;

pub mod nord;
pub mod default;
pub mod theme;
pub mod blackbird;
pub mod dracula;

pub struct ColorScheme {
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
    memory_used_memory_text_fg: Color,
    memory_used_memory_sparkline_fg: Color,
    memory_used_memory_sparkline_baseline_fg: Color,
    memory_rss_memory_text_fg: Color,
    memory_rss_memory_sparkline_fg: Color,
    memory_rss_memory_sparkline_baseline_fg: Color,

    cpu_title_fg: Color,
    cpu_border_fg: Color,
    cpu_chart_line_fg: Color,
    cpu_chart_axis_fg: Color,
    cpu_sys_cpu_text_1_fg: Color,
    cpu_sys_cpu_text_2_fg: Color,
    cpu_sys_cpu_dataset_fg: Color,
    cpu_user_cpu_text_1_fg: Color,
    cpu_user_cpu_text_2_fg: Color,
    cpu_user_cpu_dataset_fg: Color,

    hit_rate_title_fg: Color,
    hit_rate_border_fg: Color,
    hit_rate_label_fg: Color,
    hit_rate_gauge_fg: Color,
    hit_rate_gauge_bg: Color,

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

    stat_title_fg: Color,
    stat_border_fg: Color,
    stat_table_header_fg: Color,
    stat_table_row_top_1_fg: Color,
    stat_table_row_top_2_fg: Color,
    stat_table_row_bottom_fg: Color,
    stat_table_row_highlight_bg: Color,

    calls_title_fg: Color,
    calls_border_fg: Color,
    calls_table_header_fg: Color,
    calls_table_row_gauge_fg: Color,
    calls_table_row_gauge_bg: Color,
    calls_table_row_top_1_fg: Color,
    calls_table_row_top_2_fg: Color,
    calls_table_row_bottom_fg: Color,
    calls_table_row_highlight_bg: Color,

    warning_title_fg: Color,
    warning_border_fg: Color,
    warning_text_1_fg: Color,
    warning_text_2_fg: Color,
}

impl From<&str> for ColorScheme {
    fn from(s: &str) -> Self {
        match s {
            "blackbird" => blackbird::new(),
            "nord" => nord::new(),
            "dracula" => dracula::new(),
            _ => default::new()
        }
    }
}