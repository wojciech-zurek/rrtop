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
                Constraint::Length(12),
                Constraint::Min(3),
            ]
                .as_ref(),
        )
        .split(area);

    draw_top(f, chunks[0], app);
    draw_middle(f, chunks[1], app);
    draw_bottom(f, chunks[2], app);
}

fn draw_top(f: &mut Frame<Backend>, area: Rect, app: &App) {
    f.render_widget(&app.cpu, area);
}

fn draw_middle(f: &mut Frame<Backend>, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
                .as_ref(),
        )
        .split(area);

    f.render_widget(&app.network, chunks[0]);
    f.render_widget(&app.memory, chunks[1]);
}

fn draw_bottom(f: &mut Frame<Backend>, area: Rect, app: &mut App) {
    f.render_widget(&mut app.stat, area);
}
