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

        // Score when ball goes out of bounds
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
        self.ball_dy = BALL_SPEED * 0.5;
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        
        // Create main block
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Pong Game ")
            .style(Style::default().fg(Color::White));
        
        frame.render_widget(block, area);
        
        // Get inner area (excluding borders)
        let inner_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };
        
        // Render score
        let score_text = format!("Player 1: {}  |  Player 2: {}", self.score1, self.score2);
        let score_paragraph = Paragraph::new(score_text)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        
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
        
        // Render paddles and ball
        self.render_game_objects(frame, game_area);
        
        // Render game over message if applicable
        if self.game_over {
            self.render_game_over(frame, area);
        }
        
        // Render instructions
        self.render_instructions(frame, area);
    }
    
    fn render_game_objects(&self, frame: &mut Frame, area: Rect) {
        // Render left paddle (Player 1)
        let paddle1_y = (self.paddle1_y as u16).saturating_sub(PADDLE_HEIGHT / 2);
        for i in 0..PADDLE_HEIGHT {
            if paddle1_y + i < area.height && area.y + paddle1_y + i < frame.area().height {
                let paddle_area = Rect {
                    x: area.x,
                    y: area.y + paddle1_y + i,
                    width: PADDLE_WIDTH,
                    height: 1,
                };
                let paddle_char = Paragraph::new("█")
                    .style(Style::default().fg(Color::Blue));
                frame.render_widget(paddle_char, paddle_area);
            }
        }
        
        // Render right paddle (Player 2)
        let paddle2_y = (self.paddle2_y as u16).saturating_sub(PADDLE_HEIGHT / 2);
        for i in 0..PADDLE_HEIGHT {
            if paddle2_y + i < area.height && area.y + paddle2_y + i < frame.area().height {
                let paddle_area = Rect {
                    x: area.x + area.width.saturating_sub(PADDLE_WIDTH),
                    y: area.y + paddle2_y + i,
                    width: PADDLE_WIDTH,
                    height: 1,
                };
                let paddle_char = Paragraph::new("█")
                    .style(Style::default().fg(Color::Red));
                frame.render_widget(paddle_char, paddle_area);
            }
        }
        
        // Render ball
        let ball_x = self.ball_x as u16;
        let ball_y = self.ball_y as u16;
        
        if ball_x < area.width && ball_y < area.height {
            let ball_area = Rect {
                x: area.x + ball_x,
                y: area.y + ball_y,
                width: 1,
                height: 1,
            };
            let ball_char = Paragraph::new("●")
                .style(Style::default().fg(Color::White));
            frame.render_widget(ball_char, ball_area);
        }
        
        // Render center line
        let center_x = area.x + area.width / 2;
        for y in 0..area.height {
            if y % 2 == 0 {
                let line_area = Rect {
                    x: center_x,
                    y: area.y + y,
                    width: 1,
                    height: 1,
                };
                let line_char = Paragraph::new("|")
                    .style(Style::default().fg(Color::DarkGray));
                frame.render_widget(line_char, line_area);
            }
        }
    }
    
    fn render_game_over(&self, frame: &mut Frame, area: Rect) {
        let message = match self.winner {
            Some(1) => "Player 1 Wins! Press 'q' to quit.",
            Some(2) => "Player 2 Wins! Press 'q' to quit.",
            _ => "Game Over! Press 'q' to quit.",
        };
        
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 2,
            width: area.width / 2,
            height: 3,
        };
        
        let popup = Paragraph::new(message)
            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Yellow))
            );
        
        frame.render_widget(popup, popup_area);
    }
    
    fn render_instructions(&self, frame: &mut Frame, area: Rect) {
        let instructions = "Player 1: W/S  |  Player 2: ↑/↓  |  Quit: Q";
        let instructions_paragraph = Paragraph::new(instructions)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        
        let instructions_area = Rect {
            x: area.x + 1,
            y: area.y + area.height.saturating_sub(1),
            width: area.width.saturating_sub(2),
            height: 1,
        };
        
        frame.render_widget(instructions_paragraph, instructions_area);
    }
}