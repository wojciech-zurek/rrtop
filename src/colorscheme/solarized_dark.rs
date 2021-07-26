use tui::style::Color;

use crate::colorscheme::ColorScheme;
use crate::colorscheme::solarized_dark::ColorPalette::*;

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
    Violet,
}

impl From<ColorPalette> for Color {
    fn from(n: ColorPalette) -> Self {
        match n {
            White => Color::Rgb(248, 248, 242),
            Cyan => Color::Rgb(42, 161, 152),
            Red => Color::Rgb(211, 1, 2),
            Green => Color::Rgb(133, 153, 0),
            Yellow => Color::Rgb(181, 137, 0),
            Blue => Color::Rgb(38, 139, 210),
            Orange => Color::Rgb(203, 75, 22),
            Black1 => Color::Rgb(0, 43, 54),
            Black2 => Color::Rgb(7, 54, 66),
            Magenta => Color::Rgb(211, 54, 130),
            Violet => Color::Rgb(108, 113, 196),
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
        memory_border_fg: Black2.into(),
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
        hit_rate_border_fg: Black2.into(),
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
        network_border_fg: Black2.into(),
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
        stat_table_row_gauge_fg: Blue.into(),
        stat_table_row_gauge_bg: Black2.into(),
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