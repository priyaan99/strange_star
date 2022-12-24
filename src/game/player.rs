use macroquad::prelude::*;
use super::{BACKGROUND_COLOR, PLAYER_COLOR};

pub struct Player {
    pub position: Vec2,
    velocity: Vec2,
    pub dashing: bool,
    dash_dir: Vec2,
    timer: f32,
}

impl Player {
    const SPEED: f32 = 200.0;
    const DASH_SPEED: f32 = 500.0;
    pub const RADIUS: f32 = 14.0;
    const DASH_DURATION: f32 = 0.12;

    pub fn init() -> Self {
        Self {
            position: Vec2 { x: screen_width()/2., y: screen_height()/2. },
            velocity: Vec2::ZERO,
            dashing: false,
            dash_dir: Vec2::X,
            timer: 0.0,
        }
    }

    fn dash(&mut self) {
        self.timer += get_frame_time();

        if self.timer >= Self::DASH_DURATION {
            self.timer = 0.0;
            self.dashing = false;
            return;
        } 

        self.position += self.dash_dir * Self::DASH_SPEED * get_frame_time();
    }

    pub fn update(&mut self) {

        if is_key_down(KeyCode::W) {
            self.velocity = Vec2 { x: 0., y: -1.};
        } else if is_key_down(KeyCode::S) {
            self.velocity = Vec2 { x: 0., y: 1.};
        } else if is_key_down(KeyCode::A) {
            self.velocity = Vec2 { x: -1., y: 0.};
        } else if is_key_down(KeyCode::D) {
            self.velocity = Vec2 { x: 1., y: 0.};
        } else {
            self.velocity = Vec2::ZERO;
        }

        if is_key_pressed(KeyCode::Space) && !self.dashing {
            self.dashing = true;
        } 

        if self.dashing {
            self.dash()
        } else {
            self.position += self.velocity * Self::SPEED * get_frame_time();

            if self.velocity != Vec2::ZERO {
                self.dash_dir = self.velocity;
            }
        }

        self.position = self.position.clamp(Vec2::ZERO, Vec2 { x: screen_width(), y: screen_height()})
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, Self::RADIUS, PLAYER_COLOR);
        draw_circle(self.position.x, self.position.y, Self::RADIUS-4.0, BACKGROUND_COLOR);
        draw_circle(self.position.x, self.position.y, Self::RADIUS-6.0, PLAYER_COLOR);
    }
}
