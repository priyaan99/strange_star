use macroquad::prelude::*;
use strange_star::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Strange Star".to_owned(),
        fullscreen: false,
        window_width: 256,
        window_height: 256,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut app = game::State::init();

    loop {
        app.run();
        next_frame().await
    }
}
