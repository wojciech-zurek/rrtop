use tui::{Frame};
use crate::app::App;
use tui::layout::{Layout, Direction, Constraint, Rect};
use crate::terminal::{Backend, Terminal};

pub mod home;

pub fn draw(terminal: &mut Terminal, app: &App) -> std::io::Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Length(1),
                ]
                    .as_ref(),
            )
            .split(f.size());

        match app.selected_tab {
            0 => { home::draw(f, chunks[0], app) }
            _ => {}
        }

        draw_status_bar(f, chunks[1], app);
    })?;

    Ok(())
}

fn draw_status_bar(f: &mut Frame<Backend>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
            ]
                .as_ref(),
        )
        .split(area);


    f.render_widget(&app.menu, chunks[0]);
    f.render_widget(&app.status_bar, chunks[1]);
}