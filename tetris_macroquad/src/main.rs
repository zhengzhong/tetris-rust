use macroquad::prelude::*;

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
