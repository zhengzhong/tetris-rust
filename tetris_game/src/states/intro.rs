use rand::random;

use crate::conf::TetrisSettings;
use crate::{Button, Color, GamePad, GameUI, Position};

use super::{State, StateName};

pub struct Intro {
    loop_count: i32,
    bricks: Vec<Position>,
    color: Color,
    is_started: bool,
}

impl Intro {
    pub fn new(_settings: &TetrisSettings) -> Self {
        let tetris_positions = vec![
            // T
            (1, 1),
            (2, 1),
            (3, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            // E
            (5, 1),
            (6, 1),
            (5, 2),
            (5, 3),
            (5, 4),
            (5, 5),
            (6, 5),
            (6, 3),
            // t
            (9, 2),
            (9, 3),
            (9, 4),
            (9, 5),
            (10, 5),
            (8, 3),
            (10, 3),
            // r
            (12, 3),
            (12, 4),
            (12, 5),
            (13, 3),
            // i
            (15, 1),
            (15, 3),
            (15, 4),
            (15, 5),
            // s
            (18, 2),
            (17, 2),
            (17, 3),
            (18, 4),
            (18, 5),
            (17, 5),
        ];
        let bricks: Vec<Position> = tetris_positions
            .into_iter()
            .map(|(x, y)| Position::new(x, y))
            .collect();
        Self {
            loop_count: 0,
            bricks,
            color: Color::Gray,
            is_started: false,
        }
    }
}

impl State for Intro {
    fn start_loop(&mut self) {
        self.loop_count += 1;
    }

    fn process_input(&mut self, pad: &dyn GamePad) {
        if pad.is_pressed(Button::Start) {
            log::info!("Starting game");
            self.is_started = true;
        }
    }

    fn update(&mut self) {
        if self.loop_count % 20 == 0 {
            self.color = pick_random_color();
        }
    }

    fn draw(&self, ui: &mut dyn GameUI) {
        for pos in &self.bricks {
            ui.draw_brick(*pos, self.color);
        }
        ui.draw_text(Position::new(5, 8), "Start Game");
    }

    fn end_loop(&self) -> Option<StateName> {
        if self.is_started {
            log::info!("Transitioning state: Intro to Ongoing");
            Some(StateName::Ongoing)
        } else {
            None
        }
    }
}

fn pick_random_color() -> Color {
    let n: u8 = random();
    match n % 7 {
        0 => Color::Teal,
        1 => Color::Yellow,
        2 => Color::Purple,
        3 => Color::Blue,
        4 => Color::Orange,
        5 => Color::Green,
        6 => Color::Red,
        _ => panic!("Impossible!"),
    }
}
