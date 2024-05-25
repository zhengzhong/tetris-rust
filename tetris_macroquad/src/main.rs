use macroquad::prelude::*;
use tetris_macroquad::{load_settings, play_game};

fn window_conf() -> Conf {
    // Load the game settings to compute window size, then drop it.
    // We need to build `Conf` before entering `main()`. I could not find
    // a way to pass the loaded settings to `main()`, so I just drop it
    // and let `main()` load the settings again.
    let (window_width, window_height) = load_settings()
        .expect("Fail to load settings")
        .window_size();
    Conf {
        window_title: String::from("Tetris"),
        fullscreen: false,
        window_width,
        window_height,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let settings = load_settings().expect("Fail to load settings");
    play_game(settings).await
}
