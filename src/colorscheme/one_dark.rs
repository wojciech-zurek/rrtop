use tui::style::Color;

use crate::colorscheme::ColorScheme;
use crate::colorscheme::one_dark::ColorPalette::*;

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
    Purple,
}

impl From<ColorPalette> for Color {
    fn from(n: ColorPalette) -> Self {
        match n {
            White => Color::Rgb(171, 178, 191),
            Cyan => Color::Rgb(86, 182, 194),
            Red => Color::Rgb(224, 108, 117),
            Green => Color::Rgb(152, 195, 121),
            Yellow => Color::Rgb(209, 154, 102),
            Blue => Color::Rgb(97, 175, 239),
            Black1 => Color::Rgb(40, 44, 52),
            Black2 => Color::Rgb(92, 99, 112),
            Purple => Color::Rgb(198, 120, 221),
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
        menu_highlight_fg: Purple.into(),
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
        cpu_chart_axis_fg: Black2.into(),
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
        network_rx_total_text_fg: Red.into(),
        network_rx_s_text_fg: Red.into(),
        network_rx_sparkline_fg: Red.into(),
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
        stat_table_row_highlight_bg: Purple.into(),

        calls_title_fg: White.into(),
        calls_border_fg: Black2.into(),
        calls_table_header_fg: Cyan.into(),
        calls_table_row_gauge_fg: Blue.into(),
        calls_table_row_gauge_bg: Black2.into(),
        calls_table_row_top_1_fg: White.into(),
        calls_table_row_top_2_fg: Blue.into(),
        calls_table_row_bottom_fg: Black2.into(),
        calls_table_row_highlight_bg: Purple.into(),

        warning_title_fg: White.into(),
        warning_border_fg: Black2.into(),
        warning_text_1_fg: Red.into(),
        warning_text_2_fg: Green.into(),
    }
}