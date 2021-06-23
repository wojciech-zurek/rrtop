use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
use std::io::{stdout, Stdout};
use tui::backend::CrosstermBackend;
use crate::error::RRTopError;

pub type Backend = CrosstermBackend<Stdout>;
pub type Terminal = tui::Terminal<Backend>;

pub fn create() -> Result<Terminal, RRTopError> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}

pub fn clean(mut terminal: Terminal) -> Result<(), RRTopError> {
    terminal.clear()?;
    disable_raw_mode()?;
    execute!( terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}