use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::app::App;
use crate::terminal::Backend;

pub fn draw(f: &mut Frame<Backend>, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(13),
                Constraint::Max(0),
            ]
                .as_ref(),
        )
        .split(area);
    f.render_widget(&app.commands, chunks[0]);
}