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

        // Ball goes out of bounds (scoring)
        if self.ball_x < 0.0 {
            self.score2 += 1;
            self.reset_ball();
            if self.score2 >= WINNING_SCORE {
                self.game_over = true;
                self.winner = Some(2);
            }
        } else if self.ball_x > self.width as f64 {
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
        // Randomize direction a bit
        self.ball_dx = if self.ball_dx > 0.0 { -BALL_SPEED } else { BALL_SPEED };
        self.ball_dy = BALL_SPEED * 0.5;
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.game_over {
            self.render_game_over(frame, area);
            return;
        }

        // Create a block for the game area
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Pong Game")
            .title_alignment(Alignment::Center);
        
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Update dimensions if area has changed
        let new_width = inner.width;
        let new_height = inner.height;
        
        // Render score
        let score_text = format!("Player 1: {}  Player 2: {}", self.score1, self.score2);
        let score_paragraph = Paragraph::new(score_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
        
        let score_area = Rect {
            x: inner.x,
            y: inner.y,
            width: inner.width,
            height: 1,
        };
        frame.render_widget(score_paragraph, score_area);

        // Adjust game area to account for score display
        let game_area = Rect {
            x: inner.x,
            y: inner.y + 1,
            width: inner.width,
            height: inner.height.saturating_sub(1),
        };

        // Render ball
        let ball_x = (self.ball_x as u16).min(game_area.width.saturating_sub(1));
        let ball_y = (self.ball_y as u16).min(game_area.height.saturating_sub(1));
        
        if ball_x < game_area.width && ball_y < game_area.height {
            let ball_area = Rect {
                x: game_area.x + ball_x,
                y: game_area.y + ball_y,
                width: 1,
                height: 1,
            };
            let ball_widget = Paragraph::new("●")
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(ball_widget, ball_area);
        }

        // Render paddles
        let paddle1_start_y = (self.paddle1_y - PADDLE_HEIGHT as f64 / 2.0).max(0.0) as u16;
        let paddle2_start_y = (self.paddle2_y - PADDLE_HEIGHT as f64 / 2.0).max(0.0) as u16;
        
        // Left paddle
        for i in 0..PADDLE_HEIGHT {
            let paddle_y = paddle1_start_y + i;
            if paddle_y < game_area.height {
                let paddle_area = Rect {
                    x: game_area.x,
                    y: game_area.y + paddle_y,
                    width: PADDLE_WIDTH,
                    height: 1,
                };
                let paddle_widget = Paragraph::new("█")
                    .style(Style::default().fg(Color::Blue));
                frame.render_widget(paddle_widget, paddle_area);
            }
        }
        
        // Right paddle
        for i in 0..PADDLE_HEIGHT {
            let paddle_y = paddle2_start_y + i;
            if paddle_y < game_area.height {
                let paddle_area = Rect {
                    x: game_area.x + game_area.width.saturating_sub(PADDLE_WIDTH),
                    y: game_area.y + paddle_y,
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
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Game Over")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red));
        
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let message = match self.winner {
            Some(1) => "Player 1 Wins! Press 'q' to quit.",
            Some(2) => "Player 2 Wins! Press 'q' to quit.",
            None => "Game Over! Press 'q' to quit.",
        };

        let game_over_paragraph = Paragraph::new(message)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
        
        let text_area = Rect {
            x: inner.x,
            y: inner.y + inner.height / 2,
            width: inner.width,
            height: 1,
        };
        frame.render_widget(game_over_paragraph, text_area);

        // Show final score
        let final_score = format!("Final Score - Player 1: {}  Player 2: {}", self.score1, self.score2);
        let score_paragraph = Paragraph::new(final_score)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        
        let score_area = Rect {
            x: inner.x,
            y: inner.y + inner.height / 2 + 2,
            width: inner.width,
            height: 1,
        };
        frame.render_widget(score_paragraph, score_area);
    }
}