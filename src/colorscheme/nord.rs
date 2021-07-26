use tui::style::Color;

use crate::colorscheme::ColorScheme;
use crate::colorscheme::nord::ColorPalette::*;

#[allow(dead_code)]
enum ColorPalette {
    Nord0,
    Nord1,
    Nord2,
    Nord3,
    Nord4,
    Nord5,
    Nord6,
    Nord7,
    Nord8,
    Nord9,
    Nord10,
    Nord11,
    Nord12,
    Nord13,
    Nord14,
    Nord15,
}

impl From<ColorPalette> for Color {
    fn from(n: ColorPalette) -> Self {
        match n {
            Nord0 => Color::Rgb(46, 52, 64),
            Nord1 => Color::Rgb(59, 66, 82),
            Nord2 => Color::Rgb(67, 76, 94),
            Nord3 => Color::Rgb(76, 86, 106),
            Nord4 => Color::Rgb(216, 222, 233),
            Nord5 => Color::Rgb(229, 233, 240),
            Nord6 => Color::Rgb(236, 239, 244),
            Nord7 => Color::Rgb(143, 188, 187),
            Nord8 => Color::Rgb(136, 192, 208),
            Nord9 => Color::Rgb(129, 161, 193),
            Nord10 => Color::Rgb(94, 129, 172),
            Nord11 => Color::Rgb(191, 97, 106),
            Nord12 => Color::Rgb(208, 135, 112),
            Nord13 => Color::Rgb(235, 203, 139),
            Nord14 => Color::Rgb(163, 190, 140),
            Nord15 => Color::Rgb(180, 142, 173)
        }
    }
}

pub fn new() -> ColorScheme {
    ColorScheme {
        main_bg: Nord0.into(),
        main_fg: Nord4.into(),

        menu_bg: Nord0.into(),
        menu_fg: Nord3.into(),
        menu_divider_fg: Nord3.into(),
        menu_highlight_bg: Nord0.into(),
        menu_highlight_fg: Nord9.into(),

        status_bar_fg: Nord3.into(),

        memory_title_fg: Nord7.into(),
        memory_border_fg: Nord3.into(),
        memory_max_memory_text_fg: Nord11.into(),
        memory_used_memory_text_fg: Nord9.into(),
        memory_used_memory_sparkline_fg: Nord9.into(),
        memory_used_memory_sparkline_baseline_fg: Nord3.into(),
        memory_rss_memory_text_fg: Nord14.into(),
        memory_rss_memory_sparkline_fg: Nord14.into(),
        memory_rss_memory_sparkline_baseline_fg: Nord3.into(),

        cpu_title_fg: Nord7.into(),
        cpu_border_fg: Nord3.into(),
        cpu_chart_line_fg: Nord3.into(),
        cpu_chart_axis_fg: Nord3.into(),
        cpu_sys_cpu_text_1_fg: Nord3.into(),
        cpu_sys_cpu_text_2_fg: Nord3.into(),
        cpu_sys_cpu_dataset_fg: Nord11.into(),
        cpu_user_cpu_text_1_fg: Nord3.into(),
        cpu_user_cpu_text_2_fg: Nord3.into(),
        cpu_user_cpu_dataset_fg: Nord9.into(),

        hit_rate_title_fg: Nord7.into(),
        hit_rate_border_fg: Nord3.into(),
        hit_rate_label_fg: Nord9.into(),
        hit_rate_gauge_fg: Nord9.into(),
        hit_rate_gauge_bg: Nord3.into(),

        throughput_title_fg: Nord7.into(),
        throughput_border_fg: Nord3.into(),
        throughput_total_commands_text_fg: Nord9.into(),
        throughput_ops_text_fg: Nord9.into(),
        throughput_sparkline_fg: Nord9.into(),
        throughput_sparkline_baseline_fg: Nord3.into(),

        network_title_fg: Nord7.into(),
        network_border_fg: Nord3.into(),
        network_rx_total_text_fg: Nord9.into(),
        network_rx_s_text_fg: Nord9.into(),
        network_rx_sparkline_fg: Nord9.into(),
        network_rx_sparkline_baseline_fg: Nord3.into(),
        network_tx_total_text_fg: Nord14.into(),
        network_tx_s_text_fg: Nord14.into(),
        network_tx_sparkline_fg: Nord14.into(),
        network_tx_sparkline_baseline_fg: Nord3.into(),

        stat_title_fg: Nord7.into(),
        stat_border_fg: Nord3.into(),
        stat_table_header_fg: Nord7.into(),
        stat_table_row_gauge_fg: Nord9.into(),
        stat_table_row_gauge_bg: Nord1.into(),
        stat_table_row_top_1_fg: Nord6.into(),
        stat_table_row_top_2_fg: Nord9.into(),
        stat_table_row_bottom_fg: Nord3.into(),
        stat_table_row_highlight_bg: Nord2.into(),

        calls_title_fg: Nord7.into(),
        calls_border_fg: Nord3.into(),
        calls_table_header_fg: Nord7.into(),
        calls_table_row_gauge_fg: Nord9.into(),
        calls_table_row_gauge_bg: Nord1.into(),
        calls_table_row_top_1_fg: Nord6.into(),
        calls_table_row_top_2_fg: Nord9.into(),
        calls_table_row_bottom_fg: Nord3.into(),
        calls_table_row_highlight_bg: Nord2.into(),

        warning_title_fg: Nord7.into(),
        warning_border_fg: Nord3.into(),
        warning_text_1_fg: Nord11.into(),
        warning_text_2_fg: Nord14.into(),
    }
}