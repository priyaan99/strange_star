use macroquad::prelude::*;
use crate::player::*;

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
        clear_background(WHITE);
        self.player.draw();
    }
    
    pub fn run(&mut self) {
        self.update();
        self.draw();
    }
}
