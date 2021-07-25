use tui::style::{Color, Modifier, Style};

use crate::colorscheme::ColorScheme;

pub struct Theme {
    pub main: Style,

    pub menu: Style,
    pub menu_divider: Style,
    pub menu_highlight: Style,

    pub status_bar: Style,

    pub memory_title: Style,
    pub memory_border: Style,
    pub memory_chart: Style,
    pub memory_max_memory_text: Style,
    pub memory_used_memory_text: Style,
    pub memory_used_memory_sparkline: Style,
    pub memory_used_memory_sparkline_baseline: Style,
    pub memory_rss_memory_text: Style,
    pub memory_rss_memory_sparkline: Style,
    pub memory_rss_memory_sparkline_baseline: Style,

    pub cpu_title: Style,
    pub cpu_border: Style,
    pub cpu_chart: Style,
    pub cpu_chart_line: Style,
    pub cpu_chart_axis: Style,
    pub cpu_sys_cpu_1_text: Style,
    pub cpu_sys_cpu_2_text: Style,
    pub cpu_sys_cpu_dataset: Style,
    pub cpu_user_cpu_1_text: Style,
    pub cpu_user_cpu_2_text: Style,
    pub cpu_user_cpu_dataset: Style,

    pub hit_rate_title: Style,
    pub hit_rate_border: Style,
    pub hit_rate_label: Style,
    pub hit_rate_gauge: Style,

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

    pub stat_title: Style,
    pub stat_border: Style,
    pub stat_table_header: Style,
    pub stat_table_row_top_1: Style,
    pub stat_table_row_top_2: Style,
    pub stat_table_row_bottom: Style,
    pub stat_table_row_highlight: Style,

    pub calls_title: Style,
    pub calls_border: Style,
    pub calls_table_header: Style,
    pub calls_table_row_gauge: Style,
    pub calls_table_row_top_1: Style,
    pub calls_table_row_top_2: Style,
    pub calls_table_row_bottom: Style,
    pub calls_table_row_highlight: Style,

    pub warning_title: Style,
    pub warning_border: Style,
    pub warning_text_1: Style,
    pub warning_text_2: Style,

}

impl Theme {
    pub fn new(cs: ColorScheme) -> Theme {
        Theme {
            main: Style::default().fg(cs.main_fg).bg(cs.main_bg),

            menu: Style::default().fg(cs.menu_fg).bg(cs.menu_bg),
            menu_divider: Style::default().fg(cs.menu_divider_fg),
            menu_highlight: Style::default()
                .fg(cs.menu_highlight_fg)
                .bg(cs.menu_highlight_bg)
                .add_modifier(Modifier::BOLD),

            status_bar: Style::default().fg(cs.status_bar_fg),

            memory_title: Style::default().fg(cs.memory_title_fg),
            memory_border: Style::default().fg(cs.memory_border_fg),
            memory_chart: Style::default().bg(cs.main_bg),
            memory_max_memory_text: Style::default()
                .fg(cs.memory_max_memory_text_fg)
                .add_modifier(Modifier::BOLD),
            memory_used_memory_text: Style::default()
                .fg(cs.memory_used_memory_text_fg)
                .add_modifier(Modifier::BOLD),
            memory_used_memory_sparkline: Style::default().fg(cs.memory_used_memory_sparkline_fg),
            memory_used_memory_sparkline_baseline: Style::default().fg(cs.memory_used_memory_sparkline_baseline_fg),
            memory_rss_memory_text: Style::default().fg(cs.memory_rss_memory_text_fg),
            memory_rss_memory_sparkline: Style::default().fg(cs.memory_rss_memory_sparkline_fg),
            memory_rss_memory_sparkline_baseline: Style::default().fg(cs.memory_rss_memory_sparkline_baseline_fg),

            cpu_title: Style::default().fg(cs.cpu_title_fg),
            cpu_border: Style::default().fg(cs.cpu_border_fg),
            cpu_chart: Style::default().bg(cs.main_bg),
            cpu_chart_line: Style::default().fg(cs.cpu_chart_line_fg),
            cpu_chart_axis: Style::default().fg(cs.cpu_chart_axis_fg),
            cpu_sys_cpu_1_text: Style::default()
                .fg(cs.cpu_sys_cpu_text_1_fg)
                .add_modifier(Modifier::BOLD),
            cpu_sys_cpu_2_text: Style::default()
                .fg(cs.cpu_sys_cpu_text_2_fg)
                .add_modifier(Modifier::BOLD),
            cpu_sys_cpu_dataset: Style::default().fg(cs.cpu_sys_cpu_dataset_fg),
            cpu_user_cpu_1_text: Style::default()
                .fg(cs.cpu_user_cpu_text_1_fg)
                .add_modifier(Modifier::BOLD),
            cpu_user_cpu_2_text: Style::default()
                .fg(cs.cpu_user_cpu_text_2_fg)
                .add_modifier(Modifier::BOLD),
            cpu_user_cpu_dataset: Style::default().fg(cs.cpu_user_cpu_dataset_fg),

            hit_rate_title: Style::default().fg(cs.hit_rate_title_fg),
            hit_rate_border: Style::default().fg(cs.hit_rate_border_fg),
            hit_rate_label: Style::default().fg(cs.hit_rate_label_fg),
            hit_rate_gauge: Style::default().fg(cs.hit_rate_gauge_fg).bg(cs.hit_rate_gauge_bg),

            throughput_title: Style::default().fg(cs.throughput_title_fg),
            throughput_border: Style::default().fg(cs.throughput_border_fg),
            throughput_sparkline: Style::default().fg(cs.throughput_sparkline_fg),
            throughput_sparkline_baseline: Style::default().fg(cs.throughput_sparkline_baseline_fg),
            throughput_total_commands_text: Style::default()
                .fg(cs.throughput_total_commands_text_fg)
                .add_modifier(Modifier::BOLD),
            throughput_ops_text: Style::default()
                .fg(cs.throughput_ops_text_fg)
                .add_modifier(Modifier::BOLD),

            network_title: Style::default().fg(cs.network_title_fg),
            network_border: Style::default().fg(cs.network_border_fg),
            network_rx_total_text: Style::default()
                .fg(cs.network_rx_total_text_fg)
                .add_modifier(Modifier::BOLD),
            network_rx_s_text: Style::default()
                .fg(cs.network_rx_s_text_fg)
                .add_modifier(Modifier::BOLD),
            network_rx_sparkline: Style::default().fg(cs.network_rx_sparkline_fg),
            network_rx_sparkline_baseline: Style::default().fg(cs.network_rx_sparkline_baseline_fg),
            network_tx_total_text: Style::default()
                .fg(cs.network_tx_total_text_fg)
                .add_modifier(Modifier::BOLD),
            network_tx_s_text: Style::default()
                .fg(cs.network_tx_s_text_fg)
                .add_modifier(Modifier::BOLD),
            network_tx_sparkline: Style::default().fg(cs.network_tx_sparkline_fg),
            network_tx_sparkline_baseline: Style::default().fg(cs.network_tx_sparkline_baseline_fg),

            stat_title: Style::default().fg(cs.stat_title_fg),
            stat_border: Style::default().fg(cs.stat_border_fg),
            stat_table_header: Style::default()
                .fg(cs.stat_table_header_fg)
                .add_modifier(Modifier::BOLD),
            stat_table_row_top_1: Style::default().fg(cs.stat_table_row_top_1_fg).bg(cs.main_bg),
            stat_table_row_top_2: Style::default().fg(cs.stat_table_row_top_2_fg).bg(cs.main_bg),
            stat_table_row_bottom: Style::default().fg(cs.stat_table_row_bottom_fg).bg(cs.main_bg),
            stat_table_row_highlight: Style::default()
                .bg(cs.stat_table_row_highlight_bg)
                .add_modifier(Modifier::BOLD),

            calls_title: Style::default().fg(cs.calls_title_fg),
            calls_border: Style::default().fg(cs.calls_border_fg),
            calls_table_header: Style::default()
                .fg(cs.calls_table_header_fg)
                .add_modifier(Modifier::BOLD),
            calls_table_row_gauge: Style::default().fg(cs.calls_table_row_gauge_fg).bg(cs.calls_table_row_gauge_bg),
            calls_table_row_top_1: Style::default().fg(cs.calls_table_row_top_1_fg).bg(cs.main_bg),
            calls_table_row_top_2: Style::default().fg(cs.calls_table_row_top_2_fg).bg(cs.main_bg),
            calls_table_row_bottom: Style::default().fg(cs.calls_table_row_bottom_fg).bg(cs.main_bg),
            calls_table_row_highlight: Style::default()
                .bg(cs.calls_table_row_highlight_bg)
                .add_modifier(Modifier::BOLD),

            warning_title: Style::default().fg(cs.warning_title_fg),
            warning_border: Style::default().fg(cs.warning_border_fg),
            warning_text_1: Style::default().fg(cs.warning_text_1_fg),
            warning_text_2: Style::default().fg(cs.warning_text_2_fg),
        }
    }
}

impl From<&str> for Theme {
    fn from(s: &str) -> Self {
        let ch = ColorScheme::from(s);
        Theme::new(ch)
    }
}

impl Theme {
    pub fn color_table_cell(style_start: Style, style_stop: Style, index: u8, size: u16) -> Style {
        let bg = style_start.bg.unwrap_or(Color::Reset);
        let start_color = style_start.fg.unwrap_or(Color::Reset);
        let start_r: f32;
        let start_g: f32;
        let start_b: f32;

        match start_color {
            Color::Rgb(r, g, b) => {
                start_r = r as f32;
                start_g = g as f32;
                start_b = b as f32;
            }
            _ => return Style::default().fg(start_color).bg(bg)
        }

        let min = start_r.min(start_g).min(start_b) as u8;

        let stop_color = style_stop.fg.unwrap_or(Color::Rgb(min, min, min));

        let stop_r: f32;
        let stop_g: f32;
        let stop_b: f32;

        match stop_color {
            Color::Rgb(r, g, b) => {
                stop_r = r as f32;
                stop_g = g as f32;
                stop_b = b as f32;
            }
            _ => return Style::default().fg(start_color).bg(bg)
        }

        let s = match size {
            0..=12 => 12,
            13..=30 => size,
            _ => 30
        } as f32;

        let idx = index as f32;

        let r = (start_r - (((start_r - stop_r).max(0.0) / s) * idx)).max(stop_r);
        let g = (start_g - (((start_g - stop_g).max(0.0) / s) * idx)).max(stop_g);
        let b = (start_b - (((start_b - stop_b).max(0.0) / s) * idx)).max(stop_b);

        Style::default().fg(Color::Rgb(r as u8, g as u8, b as u8)).bg(bg)
    }
}
