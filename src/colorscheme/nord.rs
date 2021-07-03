use crate::colorscheme::{ColorScheme, ColorHolder};
use tui::style::{Style, Modifier, Color};

pub fn new() -> ColorHolder {
    ColorHolder {
        main_bg: Color::Rgb(46, 52, 64),
        main_fg: Color::Rgb(216, 222, 233),

        menu_bg: Color::Rgb(46, 52, 64),
        menu_fg: Color::Rgb(76, 86, 106),
        menu_divider_fg: Color::Rgb(76, 86, 106),
        menu_highlight_bg: Color::Rgb(46, 52, 64),
        menu_highlight_fg: Color::Rgb(129, 161, 193),

        status_bar_fg: Color::Rgb(76, 86, 106),

        memory_title_fg: Color::Rgb(143, 188, 187),
        memory_border_fg: Color::Rgb(76, 86, 106),
        memory_max_memory_text_fg: Color::Rgb(191, 97, 106),
        memory_max_memory_dataset_fg: Color::Rgb(191, 97, 106),
        memory_used_memory_text_fg: Color::Rgb(129, 161, 193),
        memory_used_memory_dataset_fg: Color::Rgb(129, 161, 193),

        cpu_title_fg: Color::Rgb(143, 188, 187),
        cpu_border_fg: Color::Rgb(76, 86, 106),
        cpu_chart_line_fg: Color::Rgb(76, 86, 106),
        cpu_sys_cpu_text_fg: Color::Rgb(191, 97, 106),
        cpu_sys_cpu_dataset_fg: Color::Rgb(191, 97, 106),
        cpu_user_cpu_text_fg: Color::Rgb(129, 161, 193),
        cpu_user_cpu_dataset_fg: Color::Rgb(129, 161, 193),

        throughput_title_fg: Color::Rgb(143, 188, 187),
        throughput_border_fg: Color::Rgb(76, 86, 106),
        throughput_total_commands_text_fg: Color::Rgb(129, 161, 193),
        throughput_ops_text_fg: Color::Rgb(129, 161, 193),
        throughput_sparkline_fg: Color::Rgb(129, 161, 193),
        throughput_sparkline_baseline_fg: Color::Rgb(76, 86, 106),

        network_title_fg: Color::Rgb(143, 188, 187),
        network_border_fg: Color::Rgb(76, 86, 106),
        network_rx_total_text_fg: Color::Rgb(129, 161, 193),
        network_rx_s_text_fg: Color::Rgb(129, 161, 193),
        network_rx_sparkline_fg: Color::Rgb(129, 161, 193),
        network_rx_sparkline_baseline_fg: Color::Rgb(76, 86, 106),
        network_tx_total_text_fg: Color::Rgb(163, 190, 140),
        network_tx_s_text_fg: Color::Rgb(163, 190, 140),
        network_tx_sparkline_fg: Color::Rgb(163, 190, 140),
        network_tx_sparkline_baseline_fg: Color::Rgb(76, 86, 106),
    }
}