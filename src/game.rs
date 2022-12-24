mod player;
mod stars;
mod estars;

use macroquad::prelude::*;
use player::Player;
use stars::Stars;
use estars::Estar;

const BACKGROUND_COLOR: Color = color_u8!(0x28, 0x28, 0x28, 255);
const PLAYER_COLOR: Color = color_u8!(0x0, 228, 48, 255);
const STAR_COLOR: Color = color_u8!(0xeb, 0xdb, 0xb2, 255);
const ESTAR_COLOR: Color = color_u8!(0xfa, 0xbd, 0x2f, 255);

#[inline]
fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + amount * (end - start)
}

pub struct State {
    player: Player,
    stars: Stars,
    estars: Vec<Estar>,
    min_distance: i32,
    score: i32,
    out: bool,
}

impl State {
    const MAX_ESTAR: i32 = 3;

    pub fn init() -> State {
        let player =  Player::init();
        let stars = Stars::init(200);
        let min_distance = 200;
        let mut estars = vec![];

        for _ in 0..Self::MAX_ESTAR {
            estars.push(Estar::init(player.position, min_distance))
        }

        let score = 0;
        let out = false;
        Self { out, score, player, stars, estars, min_distance}
    }

    fn update_estars(&mut self) {
        for estar in &mut self.estars {
            estar.update(self.player.position, self.min_distance)
        }
    }

    fn update(&mut self) {
        self.player.update();
        self.stars.update();
        self.update_estars();

        for estar in &mut self.estars {
            let radius_sum = estar.radius + Player::RADIUS;
            let collided = estar.position.distance_squared(self.player.position) <= (radius_sum * radius_sum);  

            if estar.active && collided {
                if self.player.dashing {
                    self.score += 5;
                    estar.destroy(self.player.position, self.min_distance)
                } else {
                    self.out = true;
                }
            }

        }
    }

    fn draw_estar(&self) {
        for estar in &self.estars {
            estar.draw();
        }
    }

    fn draw(&self) {
        clear_background(BACKGROUND_COLOR);
        self.stars.draw();
        self.player.draw();
        self.draw_estar();
    }
    
    pub fn run(&mut self) {
        self.update();
        self.draw();
    }
}
