use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Alignment};
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

    pub fn move_paddle1_up(&mut self) {
        self.paddle1_y = (self.paddle1_y - PADDLE_SPEED).max(PADDLE_HEIGHT as f64 / 2.0 + 1.0);
    }

    pub fn move_paddle1_down(&mut self) {
        self.paddle1_y = (self.paddle1_y + PADDLE_SPEED).min(self.height as f64 - PADDLE_HEIGHT as f64 / 2.0 - 1.0);
    }

    pub fn move_paddle2_up(&mut self) {
        self.paddle2_y = (self.paddle2_y - PADDLE_SPEED).max(PADDLE_HEIGHT as f64 / 2.0 + 1.0);
    }

    pub fn move_paddle2_down(&mut self) {
        self.paddle2_y = (self.paddle2_y + PADDLE_SPEED).min(self.height as f64 - PADDLE_HEIGHT as f64 / 2.0 - 1.0);
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Update ball position
        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

        // Ball collision with top and bottom walls
        if self.ball_y <= 1.0 || self.ball_y >= self.height as f64 - 1.0 {
            self.ball_dy = -self.ball_dy;
            self.ball_y = self.ball_y.clamp(1.0, self.height as f64 - 1.0);
        }

        // Ball collision with paddles
        let paddle1_top = self.paddle1_y - PADDLE_HEIGHT as f64 / 2.0;
        let paddle1_bottom = self.paddle1_y + PADDLE_HEIGHT as f64 / 2.0;
        let paddle2_top = self.paddle2_y - PADDLE_HEIGHT as f64 / 2.0;
        let paddle2_bottom = self.paddle2_y + PADDLE_HEIGHT as f64 / 2.0;

        // Left paddle collision
        if self.ball_x <= 2.0 && self.ball_dx < 0.0 {
            if self.ball_y >= paddle1_top && self.ball_y <= paddle1_bottom {
                self.ball_dx = -self.ball_dx;
                let hit_pos = (self.ball_y - self.paddle1_y) / (PADDLE_HEIGHT as f64 / 2.0);
                self.ball_dy = hit_pos * BALL_SPEED;
                self.ball_x = 2.0;
            }
        }

        // Right paddle collision
        if self.ball_x >= self.widt
... (truncated)

    }
    // Other methods omitted for brevity

    // Game over messages
    pub fn game_over_message(&self) -> &str {
        match self.winner {
            Some(1) => "Player 1 Wins\! Press 'q' to quit.",
            Some(2) => "Player 2 Wins\! Press 'q' to quit.",
            None => "Game Over\! Press 'q' to quit.",
        }
    }
}
