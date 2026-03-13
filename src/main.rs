use std::io;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, ExecutableCommand};
use ratatui::prelude::*;
use ratatui::Terminal;

mod game_state;

use game_state::GameState;

const TICK_RATE: Duration = Duration::from_millis(33); // ~30 FPS

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = restore_terminal();
        original_hook(panic);
    }));

    // Initialize terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get terminal size
    let size = terminal.size()?;
    let mut game_state = GameState::new(size.width, size.height);

    let mut last_tick = Instant::now();

    // Main game loop
    loop {
        // Handle input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('w') | KeyCode::Char('W') => game_state.move_paddle1_up(),
                    KeyCode::Char('s') | KeyCode::Char('S') => game_state.move_paddle1_down(),
                    KeyCode::Up => game_state.move_paddle2_up(),
                    KeyCode::Down => game_state.move_paddle2_down(),
                    _ => {}
                }
            }
        }

        // Update game state
        if last_tick.elapsed() >= TICK_RATE {
            game_state.update();
            last_tick = Instant::now();
        }

        // Render
        terminal.draw(|f| game_state.render(f))?;

        // Check for game over
        if game_state.is_game_over() {
            break;
        }

        // Small sleep to prevent excessive CPU usage
        std::thread::sleep(Duration::from_millis(1));
    }

    // Clean up terminal
    restore_terminal()?;
    Ok(())
}

fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
