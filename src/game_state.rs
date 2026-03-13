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

        // Ball out of bounds (scoring)
        if self.ball_x < 0.0 {
            self.score2 += 1;
            self.reset_ball();
        } else if self.ball_x > self.width as f64 {
            self.score1 += 1;
            self.reset_ball();
        }

        // Check for game over
        if self.score1 >= WINNING_SCORE {
            self.game_over = true;
            self.winner = Some(1);
        } else if self.score2 >= WINNING_SCORE {
            self.game_over = true;
            self.winner = Some(2);
        }
    }

    fn reset_ball(&mut self) {
        self.ball_x = self.width as f64 / 2.0;
        self.ball_y = self.height as f64 / 2.0;
        self.ball_dx = if self.ball_dx > 0.0 { -BALL_SPEED } else { BALL_SPEED };
        self.ball_dy = BALL_SPEED * 0.5 * if rand::random::<bool>() { 1.0 } else { -1.0 };
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let game_area = Block::default()
            .borders(Borders::ALL)
            .title("Pong")
            .style(Style::default().fg(Color::White));
        
        frame.render_widget(game_area, area);
        
        let inner = area.inner(Margin { horizontal: 1, vertical: 1 });
        
        // Render paddles
        self.render_paddle(frame, inner, 0, self.paddle1_y);
        self.render_paddle(frame, inner, inner.width - 1, self.paddle2_y);
        
        // Render ball
        self.render_ball(frame, inner);
        
        // Render score
        self.render_score(frame, inner);
        
        // Render game over message if game is over
        if self.game_over {
            self.render_game_over(frame, inner);
        }
    }

    fn render_paddle(&self, frame: &mut Frame, area: Rect, x: u16, y: f64) {
        let paddle_y = (y as u16).saturating_sub(PADDLE_HEIGHT / 2);
        
        for i in 0..PADDLE_HEIGHT {
            let row = paddle_y + i;
            if row < area.height {
                let paddle_area = Rect {
                    x: area.x + x,
                    y: area.y + row,
                    width: PADDLE_WIDTH,
                    height: 1,
                };
                
                let paddle = Paragraph::new("|")
                    .style(Style::default().fg(Color::White));
                frame.render_widget(paddle, paddle_area);
            }
        }
    }

    fn render_ball(&self, frame: &mut Frame, area: Rect) {
        let ball_x = self.ball_x as u16;
        let ball_y = self.ball_y as u16;
        
        if ball_x < area.width && ball_y < area.height {
            let ball_area = Rect {
                x: area.x + ball_x,
                y: area.y + ball_y,
                width: 1,
                height: 1,
            };
            
            let ball = Paragraph::new("●")
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(ball, ball_area);
        }
    }

    fn render_score(&self, frame: &mut Frame, area: Rect) {
        let score_text = format!("{} - {}", self.score1, self.score2);
        let score_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };
        
        let score = Paragraph::new(score_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
        frame.render_widget(score, score_area);
    }

    fn render_game_over(&self, frame: &mut Frame, area: Rect) {
        let center_y = area.height / 2;
        
        let message = match self.winner {
            Some(1) => "Player 1 Wins! Press 'q' to quit.",
            Some(2) => "Player 2 Wins! Press 'q' to quit.",
            None => "Game Over! Press 'q' to quit.",
        };
        
        let message_area = Rect {
            x: area.x,
            y: area.y + center_y,
            width: area.width,
            height: 1,
        };
        
        let game_over_msg = Paragraph::new(message)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        frame.render_widget(game_over_msg, message_area);
    }

    pub fn reset(&mut self) {
        let center_x = self.width as f64 / 2.0;
        let center_y = self.height as f64 / 2.0;
        
        self.ball_x = center_x;
        self.ball_y = center_y;
        self.ball_dx = BALL_SPEED;
        self.ball_dy = BALL_SPEED * 0.5;
        self.paddle1_y = center_y;
        self.paddle2_y = center_y;
        self.score1 = 0;
        self.score2 = 0;
        self.game_over = false;
        self.winner = None;
    }
}