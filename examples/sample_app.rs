use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::error::Error;
use termkit::EventThread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut ev = EventThread::spawn();

    loop {
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
