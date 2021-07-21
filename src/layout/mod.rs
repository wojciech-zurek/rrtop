use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::app::App;
use crate::terminal::{Backend, Term};

pub mod home;
pub mod command;
pub mod stat;
pub mod raw;

pub fn draw(terminal: &mut Term, app: &mut App) -> std::io::Result<()> {
    if let Some(style) = app.draw_background {
        let area = terminal.get_frame().size();
        terminal.current_buffer_mut().set_style(area, style);
    }

    terminal.draw(|f| {
        if f.size().width < app.min_width || f.size().height < app.min_height {
            f.render_widget(&app.area_warning, f.size());
            return;
        }

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
            1 => { command::draw(f, chunks[0], app) }
            2 => { stat::draw(f, chunks[0], app) }
            3 => { raw::draw(f, chunks[0], app) }
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
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ]
                .as_ref(),
        )
        .split(area);
    f.render_widget(&app.menu, chunks[0]);
    f.render_widget(&app.status_bar, chunks[1]);
}