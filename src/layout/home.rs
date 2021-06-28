use crate::terminal::{Terminal, Backend};
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use crate::widget::menu::Menu;
use crate::app::App;

pub fn draw(f: &mut Frame<Backend>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Ratio(4, 10),
                Constraint::Min(5),
                Constraint::Length(1),
            ]
                .as_ref(),
        )
        .split(area);

    // draw_top(f, app, chunks[0]);
    // draw_middle(f, app, chunks[1]);
    // draw_bottom(f, app, chunks[2]);
    // draw_menu(f, chunks[3], app);
}

// fn draw_menu(f: &mut Frame<Backend>, area: Rect, app: &App) {
//     f.render_widget(&app.menu, area);
// }