use tui::text::{Span, Spans};
use tui::style::Style;
use tui::symbols::line::{VERTICAL_LEFT, ROUNDED_TOP_LEFT, VERTICAL_RIGHT, ROUNDED_TOP_RIGHT, BOTTOM_LEFT, TOP_RIGHT, TOP_LEFT};

pub mod menu;
pub mod status_bar;
pub mod network;
pub mod sparkline;
pub mod throughput;
pub mod cpu;
pub mod memory;
pub mod memory_sparkline;

const DOT_SYMBOL: &str = ".";
const LINE_SYMBOL: &str = "_";

fn title(title: &str) -> String {
    format!("{} {} {}", VERTICAL_LEFT, title, VERTICAL_RIGHT)
}

fn title_span(title: &str, title_style: Style, border_style: Style) -> Spans {
    Spans::from(
        vec![
            Span::styled(format!("{} ", TOP_RIGHT), border_style),
            Span::styled(format!("{}", title), title_style),
            Span::styled(format!(" {}", TOP_LEFT), border_style)
        ]
    )
}
