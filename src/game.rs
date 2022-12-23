mod player;
mod stars;

use macroquad::prelude::*;
use player::Player;
use stars::Stars;

const BACKGROUND_COLOR: Color = color_u8!(0x28, 0x28, 0x28, 255);
const PLAYER_COLOR: Color = color_u8!(0x0, 228, 48, 255);
const STAR_COLOR: Color = color_u8!(0xeb, 0xdb, 0xb2, 255);

#[inline]
fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + amount * (end - start)
}

pub struct State {
    player: Player,
    stars: Stars,
}

impl State {

    pub fn init() -> State {
        Self {
            player: Player::init(),
            stars: Stars::init(200),
        }
    }

    fn update(&mut self) {
        self.player.update();
        self.stars.update();
    }

    fn draw(&self) {
        clear_background(BACKGROUND_COLOR);
        self.stars.draw();
        self.player.draw();
    }
    
    pub fn run(&mut self) {
        self.update();
        self.draw();
    }
}
