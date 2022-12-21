use macroquad::prelude::*;
use crate::vector::*;

pub struct Player {
    position: Vector2,
    direction: Vector2,
    radius: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2 { x: screen_width()/2., y: screen_height()/2. },
            direction: Vector2::ZERO,
            radius: 10.,
        }
    }

    pub fn update(&mut self) {

        if is_key_down(KeyCode::W) {
            self.direction = Vector2 { x: 0., y: -1.};
        } else if is_key_down(KeyCode::S) {
            self.direction = Vector2 { x: 0., y: 1.};
        } else if is_key_down(KeyCode::A) {
            self.direction = Vector2 { x: -1., y: 0.};
        } else if is_key_down(KeyCode::D) {
            self.direction = Vector2 { x: 1., y: 0.};
        } else {
            self.direction = Vector2::ZERO;
        }

        const SPEED: f32 = 5.;

        self.position = self.position.add(self.direction.mul_val(SPEED));

        self.position = self.position
            .clamp(Vector2{ x: 0., y: 0.}, Vector2 { x: screen_width(), y: screen_height()})
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }
}
