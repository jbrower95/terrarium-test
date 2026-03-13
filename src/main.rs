use ratatui::{backend::CrosstermBackend, widgets::{Block, Borders, Paragraph, BorderType}, Terminal};
use crossterm::{execute, terminal::{self, ClearType}};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Draw the UI
    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Draw border
            let border = Block::default().borders(Borders::ALL).title("Terrarium Game").border_type(BorderType::Rounded);
            f.render_widget(border, size);

            // Draw paddles
            let left_paddle = Paragraph::new("│").block(Block::default().title("Left Paddle"));
            f.render_widget(left_paddle, size.inner(&margin(1, 1, 3, 2)));

            let right_paddle = Paragraph::new("│").block(Block::default().title("Right Paddle"));
            f.render_widget(right_paddle, size.inner(&margin(1, 1, 3, 2)));

            // Draw ball
            let ball = Paragraph::new("●").block(Block::default().title("Ball"));
            f.render_widget(ball, size.inner(&margin(1, 1, 3, 2)));

            // Draw center dividing line
            let divider = Paragraph::new("-------------------").block(Block::default().title("Divider"));
            f.render_widget(divider, size.inner(&margin(1, 1, 3, 2)));

            // Draw scores
            let score = Paragraph::new("Score: 0 - 0").block(Block::default().title("Scores"));
            f.render_widget(score, size.inner(&margin(1, 1, 3, 2)));
        })?;

        // Exit on ESC key
        if let Ok(event) = crossterm::event::read() {
            match event {
                crossterm::event::Event::Key(key_event) if key_event.code == crossterm::event::KeyCode::Esc => break,
                _ => {},
            }
        }
    }

    // Disable terminal
    terminal::disable_raw_mode()?;
    Ok(())
}

fn margin(top: u16, bottom: u16, left: u16, right: u16) -> ratatui::layout::Rect {
    ratatui::layout::Rect { x: left, y: top, width: 0, height: 0 }
}
