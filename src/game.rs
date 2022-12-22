mod player;

use macroquad::prelude::*;
use player::Player;

const BACKGROUND_COLOR: Color = color_u8!(0x28, 0x28, 0x28, 255);
const PLAYER_COLOR: Color = color_u8!(0x0, 228, 48, 255);

pub struct State {
    player: Player,
}

impl State {

    pub fn init() -> State {
        Self {
            player: Player::init(),
        }
    }

    fn update(&mut self) {
        self.player.update();
    }

    fn draw(&self) {
        clear_background(BACKGROUND_COLOR);
        self.player.draw();
    }
    
    pub fn run(&mut self) {
        self.update();
        self.draw();
    }
}
