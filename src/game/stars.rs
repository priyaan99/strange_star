use macroquad::prelude::*;
use super::*;

pub struct Stars {
    stars: Vec<Star>,
}

impl Stars {
    pub fn init(num_of_stars: usize) -> Self {

        let mut stars: Vec<Star> = vec![];
        for _ in 0..num_of_stars {
            stars.push(Star::new())
        }

        Self {
            stars,
        }
    }

    pub fn update(&mut self)  {
        for star in &mut self.stars {
            star.update();
        }
    }

    pub fn draw(&self)  {
        for star in &self.stars {
            star.draw();
        }
    }
}

#[derive(Clone, Copy)]
struct Star {
    position: Vec2,
    radius: f32,
    max_radius: f32,
    reverse: bool,
    active: bool,
    blink_speed: f32,
}

impl Star {

    fn new() -> Self {
        Self { 
            position: Vec2::new(
                rand::gen_range(0.0, screen_width()) as f32, 
                rand::gen_range(0.0, screen_height()) as f32
            ), 
            radius: 0.0,
            max_radius: rand::gen_range(2.0, 5.0), 
            reverse: false,
            active: true,
            blink_speed: rand::gen_range(8.0, 30.0) / 10 as f32,
        }
    }

    fn update(&mut self) {
        if self.reverse {
            self.radius = lerp(self.radius, 0.0, self.blink_speed * get_frame_time());

            if self.radius < 0.5 {
                self.active = false;
            }
        } else {
            self.radius = lerp(self.radius, self.max_radius, self.blink_speed * get_frame_time());

            if self.radius >= self.max_radius - 0.5 {
                self.reverse = true;
            }
        }

        if !self.active {
            *self = Self::new();
        }
    }

    fn draw(&self) {
        draw_circle(
            self.position.x, 
            self.position.y,
            self.radius, 
            STAR_COLOR,
        )
    }
}
