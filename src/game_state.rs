use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
// Removed unused imports
// use ratatui::layout::{Alignment, Constraint, Direction, Layout};
// use ratatui::text::{Line, Span};
use ratatui::style::{Color, Style};

const PADDLE_HEIGHT: u16 = 5;
const PADDLE_WIDTH: u16 = 1;
const WINNING_SCORE: u32 = 10;
const BALL_SPEED: f64 = 0.8;
const PADDLE_SPEED: f64 = 1.0;

pub struct GameState {
    ball_x: f64,
    ball_y: f64,
    ball_dx: f64,
    ball_dy: f64,
    paddle1_y: f64,
    paddle2_y: f64,
    score1: u32,
    score2: u32,
    width: u16,
    height: u16,
    game_over: bool,
    winner: Option<u8>, // 1 for player 1, 2 for player 2
}

impl GameState {
    pub fn new(width: u16, height: u16) -> Self {
        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;
        
        Self {
            ball_x: center_x,
            ball_y: center_y,
            ball_dx: BALL_SPEED,
            ball_dy: BALL_SPEED * 0.5,
            paddle1_y: center_y,
            paddle2_y: center_y,
            score1: 0,
            score2: 0,
            width,
            height,
            game_over: false,
            winner: None,
        }
    }

    // ...

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        // ...

        // Check winner and prepare message
        let message = match self.winner {
            Some(1) => "Player 1 Wins!",
            Some(2) => "Player 2 Wins!",
            // Added wildcard arm to handle non-exhaustive match
            _ => "Game Over! Press 'q' to quit.",
            None => "Game is still in progress.",
        };

        // ...
    }
}