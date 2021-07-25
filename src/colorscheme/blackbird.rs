use tui::style::Color;

use crate::colorscheme::blackbird::ColorPalette::*;
use crate::colorscheme::ColorScheme;

#[allow(dead_code)]
enum ColorPalette {
    Black1,
    Black2,
    White,
    Cyan,
    Red,
    Green,
    Yellow,
    Blue,
    Orange,
    Magenta,
}

impl From<ColorPalette> for Color {
    fn from(n: ColorPalette) -> Self {
        match n {
            White => Color::Rgb(253, 247, 205),
            Cyan => Color::Rgb(0, 236, 216),
            Red => Color::Rgb(233, 39, 65),
            Green => Color::Rgb(62, 200, 64),
            Yellow => Color::Rgb(225, 220, 63),
            Blue => Color::Rgb(65, 143, 221),
            Orange => Color::Rgb(255, 153, 0),
            Black1 => Color::Rgb(0, 0, 0),
            Black2 => Color::Rgb(52, 60, 80),
            Magenta => Color::Rgb(255, 0, 204),
        }
    }
}

pub fn new() -> ColorScheme {
    ColorScheme {
        main_bg: Black1.into(),
        main_fg: White.into(),
        menu_bg: Black1.into(),
        menu_fg: White.into(),

        menu_divider_fg: Black2.into(),
        menu_highlight_bg: Black1.into(),
        menu_highlight_fg: Orange.into(),
        status_bar_fg: Blue.into(),

        memory_title_fg: White.into(),
        memory_border_fg: Yellow.into(),
        memory_max_memory_text_fg: Blue.into(),
        memory_used_memory_text_fg: Blue.into(),
        memory_used_memory_sparkline_fg: Blue.into(),
        memory_used_memory_sparkline_baseline_fg: Black2.into(),
        memory_rss_memory_text_fg: Green.into(),
        memory_rss_memory_sparkline_fg: Green.into(),
        memory_rss_memory_sparkline_baseline_fg: Black2.into(),

        cpu_title_fg: White.into(),
        cpu_border_fg: Black2.into(),
        cpu_chart_line_fg: Black2.into(),
        cpu_chart_axis_fg: White.into(),
        cpu_sys_cpu_text_1_fg: White.into(),
        cpu_sys_cpu_text_2_fg: Red.into(),
        cpu_sys_cpu_dataset_fg: Red.into(),
        cpu_user_cpu_text_1_fg: White.into(),
        cpu_user_cpu_text_2_fg: Green.into(),
        cpu_user_cpu_dataset_fg: Green.into(),

        hit_rate_title_fg: White.into(),
        hit_rate_border_fg: Orange.into(),
        hit_rate_label_fg: Blue.into(),
        hit_rate_gauge_fg: Blue.into(),
        hit_rate_gauge_bg: Black2.into(),

        throughput_title_fg: White.into(),
        throughput_border_fg: Black2.into(),
        throughput_total_commands_text_fg: Blue.into(),
        throughput_ops_text_fg: Blue.into(),
        throughput_sparkline_fg: Blue.into(),
        throughput_sparkline_baseline_fg: Black2.into(),

        network_title_fg: White.into(),
        network_border_fg: Green.into(),
        network_rx_total_text_fg: Orange.into(),
        network_rx_s_text_fg: Orange.into(),
        network_rx_sparkline_fg: Orange.into(),
        network_rx_sparkline_baseline_fg: Black2.into(),
        network_tx_total_text_fg: Blue.into(),
        network_tx_s_text_fg: Blue.into(),
        network_tx_sparkline_fg: Blue.into(),
        network_tx_sparkline_baseline_fg: Black2.into(),

        stat_title_fg: White.into(),
        stat_border_fg: Black2.into(),
        stat_table_header_fg: Cyan.into(),
        stat_table_row_top_1_fg: White.into(),
        stat_table_row_top_2_fg: Blue.into(),
        stat_table_row_bottom_fg: Black2.into(),
        stat_table_row_highlight_bg: Orange.into(),

        calls_title_fg: White.into(),
        calls_border_fg: Black2.into(),
        calls_table_header_fg: Cyan.into(),
        calls_table_row_gauge_fg: Blue.into(),
        calls_table_row_gauge_bg: Black2.into(),
        calls_table_row_top_1_fg: White.into(),
        calls_table_row_top_2_fg: Blue.into(),
        calls_table_row_bottom_fg: Black2.into(),
        calls_table_row_highlight_bg: Orange.into(),

        warning_title_fg: White.into(),
        warning_border_fg: Black2.into(),
        warning_text_1_fg: Red.into(),
        warning_text_2_fg: Green.into(),
    }
}