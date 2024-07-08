use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io;

pub fn horizontal_bar(rows: u16, color: Color) -> io::Result<()> {
    let (cols, _) = terminal::size()?;
    execute!(
        io::stdout(),
        MoveTo(0, rows),
        SetBackgroundColor(color),
        Clear(ClearType::CurrentLine),
        Print(" ".repeat(cols as usize)),
        ResetColor
    )?;
    Ok(())
}
