use std::io::{self, stdout, Stdout};

use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::{CrosstermBackend, Terminal};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init(raw_mode: bool) -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    if raw_mode {
        enable_raw_mode()?;
    }
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore(raw_mode: bool) -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    if raw_mode {
        disable_raw_mode()?;
    }
    Ok(())
}
