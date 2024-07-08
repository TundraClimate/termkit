# termkit

tui development kit using crossterm

## usage

`Cargo.toml`

```toml
termkit = { git = "https://github.com/TundraClimate/termkit" }
```

## features

- `EventThread`  
  Event listener running in a separate thread

- `enable/disable tui`  
  manage tui enable/disable

<details>
<summary>Example</summary>

```rs
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
    termkit::enable_tui()?;

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

    termkit::disable_tui()?;

    Ok(())
}
```

</details>
