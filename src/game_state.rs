pub struct GameState {
    pub ball_position: (f32, f32), // (x, y)
    pub ball_velocity: (f32, f32), // (dx, dy)
    pub paddle_positions: ((f32, f32), (f32, f32)), // (left, right)
    pub scores: (u32, u32), // (player1, player2)
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            ball_position: (0.0, 0.0),
            ball_velocity: (0.0, 0.0),
            paddle_positions: ((0.0, 0.0), (0.0, 0.0)),
            scores: (0, 0),
        }
    }

    pub fn reset_ball(&mut self) {
        self.ball_position = (0.0, 0.0);
        self.ball_velocity = (0.0, 0.0);
    }

    pub fn update(&mut self) {
        // Ball-wall collision
        if self.ball_position.1 <= 0.0 || self.ball_position.1 >= 1.0 {
            self.ball_velocity.1 *= -1.0;
        }

        // Update ball position
        self.ball_position.0 += self.ball_velocity.0;
        self.ball_position.1 += self.ball_velocity.1;

        // Check if ball is out of bounds (scoring)
        if self.ball_position.0 < 0.0 {
            self.scores.1 += 1;
            self.reset_ball();
        } else if self.ball_position.0 > 1.0 {
            self.scores.0 += 1;
            self.reset_ball();
        }
    }
}