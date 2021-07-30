use std::io::{stdout, Stdout};
use std::ops::{Deref, DerefMut};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;

use crate::error::AppError;

pub type Backend = CrosstermBackend<Stdout>;
pub type Term = tui::Terminal<Backend>;

pub struct Terminal {
    terminal: Term,
}

impl Terminal {
    pub fn new() -> Result<Self, AppError> {
        enable_raw_mode()?;

        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Term::new(backend)?;

        terminal.hide_cursor()?;
        terminal.clear()?;

        Ok(Terminal { terminal })
    }

    pub fn clean(&mut self) -> Result<(), AppError>{
        self.terminal.clear()?;
        disable_raw_mode()?;
        execute!( self.terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        Ok(())
    }

}

impl Deref for Terminal {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Terminal{
    fn deref_mut(&mut self) -> &mut Self::Target {
       &mut self.terminal
    }
}