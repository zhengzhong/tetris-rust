use std::mem;

use rand::random;

use crate::{Button, Color, GamePad, GameUI, Position};
use crate::playfield::PlayField;
use crate::tetromino::{GameWorld, Shape, Tetromino};

use super::{State, StateName};

pub struct Ongoing {
    loop_count: i32,
    fall_speed: i32,
    next_tetromino: Option<Tetromino>,
    active_tetromino: Option<Tetromino>,
    play_field: PlayField,
    score: u32,
    cheat_codes: String,
    is_game_over: bool,
    is_restarted: bool,
    is_debug_enabled: bool,
}

impl Ongoing {
    const TOP_CENTER_POS: Position = Position::new(PlayField::WIDTH / 2 - 2, 0);

    const MAX_FALL_SPEED: i32 = 20;
    const SCORE_1_ROW_DESTROYED: u32 = 10;
    const SCORE_2_ROWS_DESTROYED: u32 = 50;
    const SCORE_3_ROWS_DESTROYED: u32 = 100;
    const SCORE_4_ROWS_DESTROYED: u32 = 200;

    pub fn new() -> Self {
        Self {
            loop_count: 0,
            fall_speed: 1,
            next_tetromino: None,
            active_tetromino: None,
            play_field: PlayField::new(),
            score: 0,
            cheat_codes: String::new(),
            is_game_over: false,
            is_restarted: false,
            is_debug_enabled: false,
        }
    }

    fn fall_per_n_loops(&self) -> i32 {
        let fall_speed = if self.fall_speed < 1 {
            1 // slowest
        } else if self.fall_speed > Self::MAX_FALL_SPEED {
            Self::MAX_FALL_SPEED // fastest
        } else {
            self.fall_speed // Within 1..=Self::MAX_FALL_SPEED
        };
        1 + Self::MAX_FALL_SPEED - fall_speed
    }

    fn level(&self) -> u8 {
        let level = self.score / 100;
        if level <= u8::MAX as u32 {
            level as u8
        } else {
            u8::MAX
        }
    }

    fn take_next_tetromino(&mut self) -> Tetromino {
        // Swap in a new random tetromino into `next_tetromino`, getting its current value out.
        let mut next_tetromino = Some(
            Tetromino::new(Shape::pick(random()), Position::new(0, 0))
        );
        mem::swap(&mut self.next_tetromino, &mut next_tetromino);
        match next_tetromino {
            Some(tetromino) => Tetromino::new(tetromino.shape(), Self::TOP_CENTER_POS),
            None => Tetromino::new(Shape::pick(random()), Self::TOP_CENTER_POS),
        }
    }

    fn cheat(&mut self, cheat_codes: &str) {
        match cheat_codes {
            "solongmarianne" => {
                self.next_tetromino = Some(Tetromino::new(Shape::I, Position::new(0, 0)));
            },
            "paintitblack" => {
                self.play_field.clear();
            }
            "obladi" => {
                let rows = vec![PlayField::HEIGHT - 1];
                self.play_field.destroy_rows(&rows);
            }
            "oblada" => {
                let rows = vec![PlayField::HEIGHT - 1, PlayField::HEIGHT - 2];
                self.play_field.destroy_rows(&rows);
            }
            "hungup" => {
                self.score = 0;
            }
            "highwaytohell" => {
                self.score += 500;
            }
            _ => {
                println!("{} ?", cheat_codes);
            },
        }
    }
}

impl State for Ongoing {
    fn start_loop(&mut self) {
        if self.is_game_over {
            return;
        }
        self.loop_count += 1;
        self.fall_speed = (self.level() + 1) as i32;
        if self.active_tetromino.is_none() {
            let tetromino = self.take_next_tetromino();
            if self.play_field.is_free(tetromino.bricks()) {
                self.active_tetromino = Some(tetromino);
            } else {
                println!("Game is over!");
                self.play_field.fade_to_gray();
                self.is_game_over = true;
            }
        }
    }

    fn process_input(&mut self, pad: &dyn GamePad) {
        // Toggle debug mode: Usable no matter if game is over.
        if pad.is_pressed(Button::Select) {
            self.is_debug_enabled = !self.is_debug_enabled;
        }

        // If game is over, the only thing user can do is to restart the game.
        if self.is_game_over {
            if pad.is_pressed(Button::Start) {
                self.is_restarted = true;
            }
            return;
        }

        // Control the active tetromino.
        if let Some(tetromino) = self.active_tetromino.as_mut() {
            tetromino.move_towards(pad.direction(), &self.play_field);
            if pad.is_pressed(Button::A) {
                tetromino.rotate_right(&self.play_field);
            }
            if pad.is_pressed(Button::B) {
                tetromino.fall_to_bottom(&self.play_field);
            }
        }

        // Cheating...
        if let Some(cheat_code) = pad.cheat_code() {
            if self.cheat_codes.len() < 20 {
                self.cheat_codes.push(cheat_code);
            }
        }
        if pad.is_pressed(Button::Start) && !self.cheat_codes.is_empty() {
            let mut cheat_codes = String::new();
            mem::swap(&mut self.cheat_codes, &mut cheat_codes);
            self.cheat(&cheat_codes);
        }
    }

    fn update(&mut self) {
        if self.is_game_over {
            return;
        }
        let fall_per_n_loops = self.fall_per_n_loops();
        if self.loop_count % fall_per_n_loops == 0 {
            if let Some(tetromino) = self.active_tetromino.as_mut() {
                let has_fallen_down = tetromino.fall_down(&self.play_field);
                if !has_fallen_down {
                    // The tetromino has reached the bottom.
                    self.play_field.fill_spaces(tetromino.bricks(), tetromino.color());
                    self.active_tetromino = None;
                    let n_rows_destroyed = self.play_field.destroy_completed_rows();
                    self.score += match n_rows_destroyed {
                        0 => 0,
                        1 => Self::SCORE_1_ROW_DESTROYED,
                        2 => Self::SCORE_2_ROWS_DESTROYED,
                        3 => Self::SCORE_3_ROWS_DESTROYED,
                        _ => Self::SCORE_4_ROWS_DESTROYED,
                    };
                }
            }
        }
    }

    fn draw(&self, ui: &mut dyn GameUI) {
        ui.draw_background();

        if self.is_debug_enabled {
            ui.draw_debugging_grids();
        }

        // Draw the wall surrounding the play field.
        let wall_color = Color::Gray;
        for y in 0..=PlayField::HEIGHT {
            ui.draw_brick(Position::new(0, y), wall_color);
            ui.draw_brick(Position::new(PlayField::WIDTH + 1, y), wall_color);
        }
        for x in 0..=PlayField::WIDTH {
            ui.draw_brick(Position::new(x, PlayField::HEIGHT), wall_color);
        }

        // Draw the inactive bricks in the play field and the active tetromino.
        // Note: We move the bricks to the right by 1 unit to leave room for the left wall.
        let right_by_1 = (1, 0);
        for (position, color) in self.play_field.spaces() {
            ui.draw_brick(position.updated(right_by_1), *color);
        }
        if let Some(tetromino) = self.active_tetromino.as_ref() {
            let color = tetromino.color();
            for brick in tetromino.bricks() {
                ui.draw_brick(brick.updated(right_by_1), color);
            }
        }

        // Texts are shown on the right panel, so leave space for the play field
        // + 2 units for the wall + 2 units for left margin.
        let text_x = PlayField::WIDTH + 4;

        ui.draw_text(Position::new(text_x, 1), &format!("Score: {}", self.score));
        ui.draw_text(Position::new(text_x, 2), &format!("Level: {}", self.level()));
        ui.draw_text(Position::new(text_x, 3), &self.cheat_codes);
        if let Some(next_tetromino) = self.next_tetromino.as_ref() {
            ui.draw_text(Position::new(text_x, 4), "Next:");
            let aligned_with_text = (text_x, 5);
            let color = next_tetromino.color();
            for brick in next_tetromino.bricks() {
                ui.draw_brick(brick.updated(aligned_with_text), color);
            }
        }
        if self.is_game_over {
            ui.draw_text(Position::new(text_x, 9), "Game Over!");
        }

        if self.is_debug_enabled {
            ui.draw_text(Position::new(text_x, 11), "---- DEBUG ----");
            ui.draw_text(Position::new(text_x, 12), &format!("Loop count: {}", self.loop_count));
            ui.draw_text(Position::new(text_x, 13), &format!("Fall speed: {}", self.fall_speed));
        }
    }

    fn end_loop(&self) -> Option<StateName> {
        if self.is_game_over && self.is_restarted {
            println!("Stopping the game");
            Some(StateName::Intro)
        } else {
            None
        }
    }
}
