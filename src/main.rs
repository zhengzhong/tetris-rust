use macroquad::prelude::*;

use tetris_in_rust;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Tetris"),
        fullscreen: false,
        window_width: 960,
        window_height: 880,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    tetris_in_rust::play().await
}
