use macroquad::prelude::*;

use tetris_macroquad;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Tetris"),
        fullscreen: false,
        window_width: 960,
        window_height: 840,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    tetris_macroquad::play_game().await
}
