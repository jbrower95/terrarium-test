// main.rs

use crossterm::{event::{self, KeyEvent}, terminal::{self, ClearType}, ExecutableCommand};
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn main() -> crossterm::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::Clear(ClearType::All))?;

    let mut last_time = Instant::now();

    loop {
        if last_time.elapsed() > Duration::from_millis(100) {
            // Game logic updates
            last_time = Instant::now();
        }

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, modifiers: _, state: _ }) = event::read()? {
                match code {
                    event::KeyCode::Char('w') => {
                        // Move Player 1 paddle up
                    }
                    event::KeyCode::Char('s') => {
                        // Move Player 1 paddle down
                    }
                    event::KeyCode::Up => {
                        // Move Player 2 paddle up
                    }
                    event::KeyCode::Down => {
                        // Move Player 2 paddle down
                    }
                    event::KeyCode::Char('q') | event::KeyCode::Esc => {
                        break; // Exit the game
                    }
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
