use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

mod event_thread;
pub use event_thread::EventThread;

pub mod render;

pub fn enable_tui() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

pub fn disable_tui() -> io::Result<()> {
    execute!(io::stdout(), LeaveAlternateScreen, Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
