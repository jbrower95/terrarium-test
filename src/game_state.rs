use crossterm::{terminal::{self, ClearType}};

pub fn poll_input() -> String {
    // Placeholder for input poll implementation
    String::new()
}

pub fn update_game_state(input: String, player_score: &mut u32, enemy_score: &mut u32) {
    // Placeholder for actual game state update logic
}

pub fn render(player_score: u32, enemy_score: u32) {
    // Placeholder for rendering the game state
    println!("Player Score: {}, Enemy Score: {}", player_score, enemy_score);
}