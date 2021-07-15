use tui::style::Style;
use tui::symbols::line::{TOP_LEFT, TOP_RIGHT};
use tui::text::{Span, Spans};
use tui::widgets::TableState;

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

pub trait Navigation  {
    fn state(&mut self) -> &mut TableState;
    fn len(&self) -> usize;

    fn next(&mut self) {
        let len = self.len();
        let next = Self::next_item(self.state(), len);
        self.state().select(Some(next));
    }

    fn prev(&mut self) {
        let len = self.len();
        let prev = Self::previous(self.state(), len);
        self.state().select(Some(prev));
    }

    fn next_item(state: &TableState, len: usize) -> usize {
        match state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        }
    }

    fn previous(state: &TableState, len: usize) -> usize {
        match state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        }
    }
}


