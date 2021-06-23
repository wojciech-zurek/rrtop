use crate::terminal::{Terminal, Backend};
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use crate::widget::menu::Menu;

pub fn draw(terminal: &mut Terminal) -> std::io::Result<()> {
    terminal.draw(|f| {
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
            .split(f.size());

        // draw_top(f, app, chunks[0]);
        // draw_middle(f, app, chunks[1]);
        // draw_bottom(f, app, chunks[2]);
        draw_menu(f, chunks[3]);
    })?;

    Ok(())
}

fn draw_menu(f: &mut Frame<Backend>, area: Rect) {
    f.render_widget(&Menu::new(), area);
}