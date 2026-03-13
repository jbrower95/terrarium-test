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
        if self.ball_x >= self.width as f64 - 2.0 && self.ball_dx > 0.0 {
            if self.ball_y >= paddle2_top && self.ball_y <= paddle2_bottom {
                self.ball_dx = -self.ball_dx;
                let hit_pos = (self.ball_y - self.paddle2_y) / (PADDLE_HEIGHT as f64 / 2.0);
                self.ball_dy = hit_pos * BALL_SPEED;
                self.ball_x = self.width as f64 - 2.0;
            }
        }

        // Ball goes off left edge (Player 2 scores)
        if self.ball_x < 0.0 {
            self.score2 += 1;
            self.reset_ball();
            if self.score2 >= WINNING_SCORE {
                self.game_over = true;
                self.winner = Some(2);
            }
        }

        // Ball goes off right edge (Player 1 scores)
        if self.ball_x > self.width as f64 {
            self.score1 += 1;
            self.reset_ball();
            if self.score1 >= WINNING_SCORE {
                self.game_over = true;
                self.winner = Some(1);
            }
        }
    }

    fn reset_ball(&mut self) {
        self.ball_x = self.width as f64 / 2.0;
        self.ball_y = self.height as f64 / 2.0;
        // Random direction for next serve
        self.ball_dx = if rand::random() { BALL_SPEED } else { -BALL_SPEED };
        self.ball_dy = if rand::random() { BALL_SPEED * 0.5 } else { -BALL_SPEED * 0.5 };
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.game_over {
            self.render_game_over(frame, area);
            return;
        }

        // Create the game area with borders
        let game_block = Block::default()
            .borders(Borders::ALL)
            .title("Pong")
            .title_alignment(Alignment::Center);
        
        let inner_area = game_block.inner(area);
        frame.render_widget(game_block, area);
        
        // Render score
        let score_text = format!("Player 1: {}  |  Player 2: {}", self.score1, self.score2);
        let score_paragraph = Paragraph::new(score_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
        
        let score_area = Rect {
            x: inner_area.x,
            y: inner_area.y,
            width: inner_area.width,
            height: 1,
        };
        frame.render_widget(score_paragraph, score_area);
        
        // Game area (below score)
        let game_area = Rect {
            x: inner_area.x,
            y: inner_area.y + 2,
            width: inner_area.width,
            height: inner_area.height.saturating_sub(2),
        };
        
        // Update dimensions if area changed
        let mut game_state = self.clone();
        game_state.width = game_area.width;
        game_state.height = game_area.height;
        
        // Render paddles and ball
        game_state.render_paddles_and_ball(frame, game_area);
    }
    
    fn render_paddles_and_ball(&self, frame: &mut Frame, area: Rect) {
        // Calculate positions within the area
        let ball_screen_x = area.x + (self.ball_x as u16).min(area.width.saturating_sub(1));
        let ball_screen_y = area.y + (self.ball_y as u16).min(area.height.saturating_sub(1));
        
        let paddle1_screen_x = area.x + 1;
        let paddle1_screen_y = area.y + ((self.paddle1_y as u16).saturating_sub(PADDLE_HEIGHT / 2)).min(area.height.saturating_sub(PADDLE_HEIGHT));
        
        let paddle2_screen_x = area.x + area.width.saturating_sub(2);
        let paddle2_screen_y = area.y + ((self.paddle2_y as u16).saturating_sub(PADDLE_HEIGHT / 2)).min(area.height.saturating_sub(PADDLE_HEIGHT));
        
        // Render ball
        if ball_screen_x < area.x + area.width && ball_screen_y < area.y + area.height {
            let ball_area = Rect {
                x: ball_screen_x,
                y: ball_screen_y,
                width: 1,
                height: 1,
            };
            let ball_widget = Paragraph::new("●")
                .style(Style::default().fg(Color::White));
            frame.render_widget(ball_widget, ball_area);
        }
        
        // Render left paddle
        for i in 0..PADDLE_HEIGHT {
            if paddle1_screen_y + i < area.y + area.height {
                let paddle_area = Rect {
                    x: paddle1_screen_x,
                    y: paddle1_screen_y + i,
                    width: PADDLE_WIDTH,
                    height: 1,
                };
                let paddle_widget = Paragraph::new("█")
                    .style(Style::default().fg(Color::Blue));
                frame.render_widget(paddle_widget, paddle_area);
            }
        }
        
        // Render right paddle
        for i in 0..PADDLE_HEIGHT {
            if paddle2_screen_y + i < area.y + area.height {
                let paddle_area = Rect {
                    x: paddle2_screen_x,
                    y: paddle2_screen_y + i,
                    width: PADDLE_WIDTH,
                    height: 1,
                };
                let paddle_widget = Paragraph::new("█")
                    .style(Style::default().fg(Color::Red));
                frame.render_widget(paddle_widget, paddle_area);
            }
        }
    }
    
    fn render_game_over(&self, frame: &mut Frame, area: Rect) {
        let message = match self.winner {
            Some(1) => "Player 1 Wins! Press 'q' to quit.",
            Some(2) => "Player 2 Wins! Press 'q' to quit.",
            None => "Game Over! Press 'q' to quit.",
        };
        
        let game_over_paragraph = Paragraph::new(message)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Game Over"));
            
        frame.render_widget(game_over_paragraph, area);
    }
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        Self {
            ball_x: self.ball_x,
            ball_y: self.ball_y,
            ball_dx: self.ball_dx,
            ball_dy: self.ball_dy,
            paddle1_y: self.paddle1_y,
            paddle2_y: self.paddle2_y,
            score1: self.score1,
            score2: self.score2,
            width: self.width,
            height: self.height,
            game_over: self.game_over,
            winner: self.winner,
        }
    }
}

extern crate rand;
