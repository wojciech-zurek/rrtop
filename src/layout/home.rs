use crate::terminal::Backend;
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use crate::app::App;

pub fn draw(f: &mut Frame<Backend>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(7),
                Constraint::Min(0)
                // Constraint::Ratio(4, 10),
                // Constraint::Min(5),
                // Constraint::Length(1),
            ]
                .as_ref(),
        )
        .split(area);

    draw_top(f, chunks[0], app);
    draw_middle(f, chunks[1], app);
    draw_bottom(f, chunks[2], app);
    // draw_middle(f, app, chunks[1]);
    // draw_bottom(f, app, chunks[2]);
    // draw_menu(f, chunks[3], app);
}

fn draw_top(f: &mut Frame<Backend>, area: Rect, app: &App) {
    f.render_widget(&app.cpu, area);
}

fn draw_middle(f: &mut Frame<Backend>, area: Rect, app: &App) {
    f.render_widget(&app.network, area);
}

fn draw_bottom(f: &mut Frame<Backend>, area: Rect, app: &App) {
    f.render_widget(&app.throughput, area);
}
// fn draw_menu(f: &mut Frame<Backend>, area: Rect, app: &App) {
//     f.render_widget(&app.menu, area);
// }