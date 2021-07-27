use tui::style::Style;
use tui::symbols::line::{TOP_LEFT, TOP_RIGHT};
use tui::text::{Span, Spans};

pub mod menu;
pub mod status_bar;
pub mod network;
pub mod sparkline;
pub mod throughput;
pub mod cpu;
pub mod memory;
pub mod stat;
mod cpu_sys;
mod cpu_user;
pub mod hit_rate;
pub mod area_warning;
pub mod calls;
mod linegauge;
pub mod raw;
pub mod navigation;
pub mod slow_log;

const MIN_DOT_SYMBOL: &str = "⡀";
const LINE_SYMBOL: &str = "_";

fn title_span(title: &str, title_style: Style, border_style: Style) -> Spans {
    Spans::from(
        vec![
            Span::styled(format!("{} ", TOP_RIGHT), border_style),
            Span::styled(format!("{}", title), title_style),
            Span::styled(format!(" {}", TOP_LEFT), border_style)
        ]
    )
}


