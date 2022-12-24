use macroquad::prelude::*;
use super::{ESTAR_COLOR, lerp}; 

#[derive(Debug)]
pub struct Estars {
    pub stars: Vec<Estar>,
    player_pos: Vec2,
    min_distance: i32,
}

impl Estars {
    pub fn init(
        num_of_stars: usize,
        player_pos: Vec2,
        min_distance: i32, 
    ) -> Self {

        let mut stars: Vec<Estar> = vec![];
        for _ in 0..num_of_stars {
            stars.push(Estar::init(player_pos, min_distance))
        }

        Self {
            stars,
            player_pos,
            min_distance,
        }
    }

    pub fn update(&mut self, player_pos: Vec2)  {
        self.player_pos = player_pos;

        for star in &mut self.stars {
            star.update(self.player_pos, self.min_distance);
        }
    }

    pub fn draw(&self)  {
        for star in &self.stars {
            star.draw();
        }
    }
}

#[derive(Debug)]
pub struct Estar {
    pub position: Vec2,
    direction: Vec2,
    pub radius: f32,
    pub active: bool,
    timer: f32,
    star_time: f32,
    pub destroy: bool,
}

impl Estar {

    const SPEED: f32    = 180.0;
    const MAX_RADIUS: f32   = 16.0;
    
    pub fn init(player_pos: Vec2, min_distance: i32) -> Self {
        let position = Self::get_pos(player_pos, min_distance);  
        let direction = (player_pos - position).normalize();
        let radius = 1.0;
        let star_time = rand::gen_range(0.2, 1.6);
        Self {
            position,
            direction,
            radius,
            active: false,
            star_time,
            timer: 0.0,
            destroy: false,
        }
    }

    // runs when star is active
    pub fn update(&mut self, player_pos: Vec2, min_distance: i32) {

        self.timer += get_frame_time();

        if !self.active {
            if self.timer > self.star_time {
                self.active = true;
            }
            return;
        }

        // now active
        self.radius = lerp(self.radius, Self::MAX_RADIUS, 0.05);
        self.position += self.direction * Self::SPEED * get_frame_time();

        // player collision
        if self.destroy {
            *self = Self::init(player_pos, min_distance);
            return;
        }
        
        // out of screen
        if self.position.x < -self.radius
        || self.position.y < -self.radius
        || self.position.x > screen_width() + self.radius 
        || self.position.y > screen_height() + self.radius
        {
            *self = Self::init(player_pos, min_distance)
        }
    }

    // runs when star is active
    pub fn draw(&self) {
        if self.active {
            draw_circle (
                self.position.x,
                self.position.y,
                self.radius, 
                ESTAR_COLOR,
            )
        }
    }
}

impl Estar {
    fn random_pos() -> Vec2 {
        Vec2::new(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height()))
    }

    fn get_pos(player_pos: Vec2, min_distance: i32) -> Vec2 {
        let mut position = Self::random_pos(); 

        while position.distance_squared(player_pos) < (min_distance * min_distance) as f32 {
            position = Self::random_pos();
        }

        position
    }
}
