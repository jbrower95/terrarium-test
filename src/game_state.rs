use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::text::{Line, Span};
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

        // Scoring
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
        self.ball_dx = if self.ball_dx > 0.0 { -BALL_SPEED } else { BALL_SPEED };
        self.ball_dy = BALL_SPEED * 0.5;
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn render(&self, f: &mut Frame) {
        let area = f.area();

        // Create main game area
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Terrarium Pong")
            .title_alignment(Alignment::Center);
        let game_area = block.inner(area);
        f.render_widget(block, area);

        // Render scores
        let score_text = format!("Player 1: {}    Player 2: {}", self.score1, self.score2);
        let score_para = Paragraph::new(score_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        let score_area = Rect {
            x: game_area.x,
            y: game_area.y,
            width: game_area.width,
            height: 1,
        };
        f.render_widget(score_para, score_area);

        // Render center line
        let center_x = game_area.x + game_area.width / 2;
        for y in (game_area.y + 2..game_area.y + game_area.height).step_by(2) {
            if y < game_area.y + game_area.height {
                let line_area = Rect {
                    x: center_x,
                    y,
                    width: 1,
                    height: 1,
                };
                let line = Paragraph::new("|")
                    .style(Style::default().fg(Color::DarkGray));
                f.render_widget(line, line_area);
            }
        }

        // Render paddles
        let paddle1_start_y = (self.paddle1_y - PADDLE_HEIGHT as f64 / 2.0) as u16;
        let paddle2_start_y = (self.paddle2_y - PADDLE_HEIGHT as f64 / 2.0) as u16;

        for i in 0..PADDLE_HEIGHT {
            // Left paddle
            let paddle1_y = paddle1_start_y + i;
            if paddle1_y >= game_area.y + 2 && paddle1_y < game_area.y + game_area.height {
                let paddle1_area = Rect {
                    x: game_area.x + 1,
                    y: paddle1_y,
                    width: 1,
                    height: 1,
                };
                let paddle1 = Paragraph::new("█")
                    .style(Style::default().fg(Color::Blue));
                f.render_widget(paddle1, paddle1_area);
            }

            // Right paddle
            let paddle2_y = paddle2_start_y + i;
            if paddle2_y >= game_area.y + 2 && paddle2_y < game_area.y + game_area.height {
                let paddle2_area = Rect {
                    x: game_area.x + game_area.width - 2,
                    y: paddle2_y,
                    width: 1,
                    height: 1,
                };
                let paddle2 = Paragraph::new("█")
                    .style(Style::default().fg(Color::Red));
                f.render_widget(paddle2, paddle2_area);
            }
        }

        // Render ball
        let ball_screen_x = game_area.x + (self.ball_x as u16).saturating_sub(1);
        let ball_screen_y = game_area.y + (self.ball_y as u16).saturating_sub(1) + 1;

        if ball_screen_x < game_area.x + game_area.width - 1 && 
           ball_screen_y >= game_area.y + 2 && 
           ball_screen_y < game_area.y + game_area.height {
            let ball_area = Rect {
                x: ball_screen_x,
                y: ball_screen_y,
                width: 1,
                height: 1,
            };
            let ball = Paragraph::new("●")
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(ball, ball_area);
        }

        // Render game over message
        if self.game_over {
            let message = match self.winner {
                Some(1) => "Player 1 Wins! Press 'q' to quit.",
                Some(2) => "Player 2 Wins! Press 'q' to quit.",
                None => "Game Over! Press 'q' to quit.",
            };

            let game_over_area = Rect {
                x: game_area.x + game_area.width / 4,
                y: game_area.y + game_area.height / 2,
                width: game_area.width / 2,
                height: 3,
            };

            let game_over_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            f.render_widget(game_over_block, game_over_area);

            let inner_area = game_over_block.inner(game_over_area);
            let game_over_text = Paragraph::new(message)
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center);
            f.render_widget(game_over_text, inner_area);
        }

        // Render controls
        let controls_text = "Player 1: W/S    Player 2: ↑/↓    Quit: Q/Esc";
        let controls_para = Paragraph::new(controls_text)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        let controls_area = Rect {
            x: game_area.x,
            y: game_area.y + game_area.height - 1,
            width: game_area.width,
            height: 1,
        };
        f.render_widget(controls_para, controls_area);
    }
}
