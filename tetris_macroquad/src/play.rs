use macroquad::prelude::*; // TODO: Should not depend on macroquad
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use tetris_game::Tetris;

use crate::ui::{MacroquadGamePad, MacroquadUI};

pub async fn play_game() {
    log::info!(
        "Starting game with screen size: {} x {}",
        screen_width(),
        screen_height()
    );

    let mut ui = MacroquadUI::new();
    let mut pad = MacroquadGamePad::new();
    let mut tetris = Tetris::new();

    ui.clear_background();

    let interval = Duration::from_millis(25);
    let mut n_loops = 0;
    let mut t = SystemTime::now();
    loop {
        // `unwrap` is not safe as occasionally we can get `SystemTimeError` (I don't know why...).
        let dt = SystemTime::now().duration_since(t).unwrap_or_default();
        if interval > dt {
            let dt_to_sleep = interval - dt;
            sleep(dt_to_sleep);
        } else {
            let overrun_millis = (dt - interval).as_millis();
            log::warn!("Loop #{} overran {} millis!", n_loops, overrun_millis);
        }
        n_loops += 1;
        t = SystemTime::now();

        tetris.start_loop();
        pad.refresh_input();
        tetris.process_input(&pad);
        tetris.update();
        tetris.draw(&mut ui);
        tetris.end_loop();

        next_frame().await
    }
}
