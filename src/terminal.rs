use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
use std::io::{stdout, Stdout};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use crate::error::RRTopError;

pub fn create() -> Result<Terminal<CrosstermBackend<Stdout>>, RRTopError> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}

pub fn clean(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), RRTopError> {
    terminal.clear()?;
    disable_raw_mode()?;
    execute!( terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}