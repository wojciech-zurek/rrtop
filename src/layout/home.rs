use crate::terminal::Backend;
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use crate::app::App;

pub fn draw(f: &mut Frame<Backend>, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(12),
                Constraint::Min(12),
            ]
                .as_ref(),
        )
        .split(area);

    draw_top(f, chunks[0], app);
    draw_middle(f, chunks[1], app);
}

fn draw_top(f: &mut Frame<Backend>, area: Rect, app: &App) {
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

    f.render_widget(&app.cpu, chunks[0]);
    f.render_widget(&app.memory, chunks[1]);
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
                Constraint::Length(7),
                Constraint::Min(0),
            ]
                .as_ref(),
        )
        .split(area);
    f.render_widget(&app.network, chunks[0]);
    f.render_widget(&app.throughput, chunks[1]);
}

fn draw_part_middle_right(f: &mut Frame<Backend>, area: Rect, app: &mut App) {
    f.render_stateful_widget(&app.stat, area, &mut app.stat_table_state);
}
