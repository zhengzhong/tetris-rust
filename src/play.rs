use std::thread::sleep;
use std::time::{Duration, SystemTime};
use macroquad::prelude::*; // TODO: Should not depend on macroquad

use crate::common::Button;
use crate::tetris::Tetris;
use crate::ui::{MacroquadGamePad, MacroquadUI};

pub async fn play() {
    println!("screen size: {} x {}", screen_width(), screen_height());

    let mut ui = MacroquadUI::new();
    let mut pad = MacroquadGamePad::new();
    let mut tetris = Tetris::new();

    ui.clear_background();
    tetris.start_game();

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
            println!("[WARN] Loop #{} overran {} millis!", n_loops, overrun_millis);
        }
        n_loops += 1;
        t = SystemTime::now();

        vec![
            (KeyCode::Left, Button::Left),
            (KeyCode::Right, Button::Right),
            (KeyCode::Up, Button::Up),
            (KeyCode::Down, Button::Down),
            (KeyCode::Space, Button::A),
            (KeyCode::Tab, Button::B),
            (KeyCode::LeftControl, Button::Select),
            (KeyCode::Enter, Button::Start),
        ]
            .into_iter()
            .for_each(|(key, button)| {
                pad.set_pressed(button, is_key_pressed(key));
            });
        pad.set_cheat_code(get_char_pressed());

        tetris.start_loop();
        tetris.process_input(&pad);
        tetris.update_state();
        tetris.draw(&mut ui);
        tetris.end_loop();

        next_frame().await
    }
}
