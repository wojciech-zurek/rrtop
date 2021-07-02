use tui::text::{Span, Spans};
use tui::style::Style;

pub mod menu;
pub mod status_bar;
pub mod network;
pub mod sparkline;
pub mod throughput;
pub mod cpu;
pub mod memory;
pub mod memory_sparkline;


const H_LINE: &str = "─";
const V_LINE: &str = "│";
const LEFT_UP: &str = "┌";
const RIGHT_UP: &str = "┐";
const LEFT_DOWN: &str = "└";
const RIGHT_DOWN: &str = "┘";
const TITLE_LEFT: &str = "┤";
const TITLE_RIGHT: &str = "├";
const DIV_UP: &str = "┬";
const DIV_DOWN: &str = "┴";

fn title(title: &str) -> String {
    format!("{} {} {}", TITLE_LEFT, title, TITLE_RIGHT)
}

fn title_span(title: &str, title_style: Style, border_style: Style) -> Spans {
    Spans::from(
        vec![
            Span::styled(format!("{} ", TITLE_LEFT), border_style),
            Span::styled(format!("{}", title), title_style),
            Span::styled(format!(" {}", TITLE_RIGHT), border_style)
        ]
    )
}
