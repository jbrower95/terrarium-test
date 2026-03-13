use std::thread;
use std::time::{Duration, Instant};
use crossterm::{execute, terminal::{self, ClearType}};

mod game_state;

const TICK_RATE_MS: u64 = 33;
const WINNING_POINTS: u32 = 10;

fn main() {
    // Initialize terminal
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, terminal::Clear(ClearType::All)).unwrap();

    let mut game_over = false;
    let mut player_score = 0;
    let mut enemy_score = 0;
    let mut last_tick = Instant::now();

    while !game_over {
        // Poll input
        let input = game_state::poll_input();
        game_state::update_game_state(input, &mut player_score, &mut enemy_score);
        
        // Check for game over condition
        if player_score >= WINNING_POINTS {
            game_over = true;
            println!("You Win!");
        } else if enemy_score >= WINNING_POINTS {
            game_over = true;
            println!("You Lose!");
        }

        // Render the game state
        game_state::render(player_score, enemy_score);

        // Sleep to maintain fixed tick rate
        let elapsed = last_tick.elapsed();
        if elapsed < Duration::from_millis(TICK_RATE_MS) {
            thread::sleep(Duration::from_millis(TICK_RATE_MS) - elapsed);
        }
        last_tick = Instant::now();
    }

    // Clean terminal restore
    terminal::disable_raw_mode().unwrap();
    execute!(stdout, terminal::Clear(ClearType::All)).unwrap();
}