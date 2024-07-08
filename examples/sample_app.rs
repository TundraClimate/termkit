use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
    style::Color,
    terminal,
};
use std::error::Error;
use termkit::{render, EventThread};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut ev = EventThread::spawn();

    loop {
        let (_, rows) = terminal::size()?;
        render::horizontal_bar(0, Color::White)?;
        render::horizontal_bar(rows, Color::White)?;
        if let Ok(event) = ev.read() {
            if let Event::Key(event) = event {
                if event.kind == KeyEventKind::Press {
                    match event.code {
                        KeyCode::Char('q') => {
                            ev.shatdown().await?;
                            break;
                        }
                        _ => ev.respond().await?,
                    }
                }
            }
        }
    }

    Ok(())
}
