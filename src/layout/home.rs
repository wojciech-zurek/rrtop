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
                Constraint::Min(12),
            ]
                .as_ref(),
        )
        .split(area);

    draw_top(f, chunks[0], app);
    draw_middle(f, chunks[1], app);
}

fn draw_top(f: &mut Frame<Backend>, area: Rect, app: &App) {
    f.render_widget(&app.cpu, area);
}

fn draw_middle(f: &mut Frame<Backend>, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ]
                .as_ref(),
        )
        .split(area);

    draw_part_middle_left(f, chunks[0], app);
    draw_part_middle_right(f, chunks[1], app);
}

fn draw_part_middle_left(f: &mut Frame<Backend>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Min(1),
            ]
                .as_ref(),
        )
        .split(area);
    f.render_widget(&app.network, chunks[0]);
    f.render_widget(&app.memory, chunks[1]);
    // f.render_widget(&app.hit_rate, chunks[2]);
}

fn draw_part_middle_right(f: &mut Frame<Backend>, area: Rect, app: &mut App) {
    f.render_widget(&mut app.stat, area);
}
