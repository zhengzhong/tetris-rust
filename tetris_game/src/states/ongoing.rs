use std::mem;

use rand::random;

use crate::conf::TetrisSettings;
use crate::playfield::PlayField;
use crate::tetromino::{GameWorld, Shape, Tetromino};
use crate::{Button, Color, GamePad, GameUI, Position};

use super::{State, StateName};

pub struct Ongoing<'a> {
    settings: &'a TetrisSettings,

    loop_count: i32,

    next_tetromino: Option<Tetromino>,
    active_tetromino: Option<Tetromino>,
    play_field: PlayField,
    score: u32,
    cheat_codes: String,
    is_game_over: bool,
    is_restarted: bool,
    is_debug_enabled: bool,
}

impl<'a> Ongoing<'a> {
    pub fn new(settings: &'a TetrisSettings) -> Self {
        Self {
            settings,

            loop_count: 0,
            next_tetromino: None,
            active_tetromino: None,
            play_field: PlayField::new(settings.play_field_width, settings.play_field_height),
            score: 0,
            cheat_codes: String::new(),
            is_game_over: false,
            is_restarted: false,
            is_debug_enabled: false,
        }
    }

    fn top_center_pos(&self) -> Position {
        Position::new(self.play_field.width() / 2 - 2, 0)
    }

    fn level(&self) -> u8 {
        let level = self.score / self.settings.score_per_level;
        if level <= u8::MAX as u32 {
            level as u8
        } else {
            u8::MAX
        }
    }

    /// The current fall pace, computed from `level`. Fall pace is the number
    /// of game loops for the tetromino to fall by one unit. The smaller the
    /// fall pace is, the faster the game speed is.
    fn fall_pace(&self) -> u8 {
        let level = self.level();
        let fall_pace = if self.settings.fall_pace_slowest > level {
            self.settings.fall_pace_slowest - level
        } else {
            0
        };
        fall_pace.max(self.settings.fall_pace_fastest)
    }

    fn take_next_tetromino(&mut self) -> Tetromino {
        // Swap in a new random tetromino into `next_tetromino`, getting its current value out.
        let mut next_tetromino = Some(Tetromino::new(Shape::pick(random()), Position::new(0, 0)));
        mem::swap(&mut self.next_tetromino, &mut next_tetromino);
        match next_tetromino {
            Some(tetromino) => Tetromino::new(tetromino.shape(), self.top_center_pos()),
            None => Tetromino::new(Shape::pick(random()), self.top_center_pos()),
        }
    }

    fn cheat(&mut self, cheat_codes: &str) {
        if !self.settings.enable_cheating {
            // Echo the cheat code, but do nothing.
            log::info!("{}", cheat_codes);
            return;
        }

        match cheat_codes {
            "solongmarianne" => {
                self.next_tetromino = Some(Tetromino::new(Shape::I, Position::new(0, 0)));
            }
            "paintitblack" => {
                self.play_field.clear();
            }
            "obladi" => {
                let rows = vec![self.play_field.height() - 1];
                self.play_field.destroy_rows(&rows);
            }
            "oblada" => {
                let rows = vec![self.play_field.height() - 1, self.play_field.height() - 2];
                self.play_field.destroy_rows(&rows);
            }
            "hungup" => {
                self.score = 0;
            }
            "highwaytohell" => {
                self.score += 500;
            }
            _ => {
                log::info!("{} ?", cheat_codes);
            }
        }
    }
}

impl<'a> State for Ongoing<'a> {
    fn start_loop(&mut self) {
        if self.is_game_over {
            return;
        }
        self.loop_count += 1;
        if self.active_tetromino.is_none() {
            let tetromino = self.take_next_tetromino();
            if self.play_field.is_free(tetromino.bricks()) {
                self.active_tetromino = Some(tetromino);
            } else {
                log::info!("No free space for new tetromino: Game is over!");
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
        let fall_pace = self.fall_pace();
        if self.loop_count % (fall_pace as i32) == 0 {
            if let Some(tetromino) = self.active_tetromino.as_mut() {
                let has_fallen_down = tetromino.fall_down(&self.play_field);
                if !has_fallen_down {
                    // The tetromino has reached the bottom.
                    self.play_field
                        .fill_space(tetromino.bricks(), tetromino.color());
                    self.active_tetromino = None;
                    let n_rows_destroyed = self.play_field.destroy_completed_rows();
                    self.score += if n_rows_destroyed > 0 {
                        let max_index = self.settings.scores_for_rows_destroyed.len() - 1;
                        let index = max_index.min((n_rows_destroyed - 1) as usize);
                        self.settings.scores_for_rows_destroyed[index]
                    } else {
                        0
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
        for y in 0..=self.play_field.height() {
            ui.draw_brick(Position::new(0, y), wall_color);
            ui.draw_brick(Position::new(self.play_field.width() + 1, y), wall_color);
        }
        for x in 0..=self.play_field.width() {
            ui.draw_brick(Position::new(x, self.play_field.height()), wall_color);
        }

        // Draw the inactive bricks in the play field and the active tetromino.
        // Note: We move the bricks to the right by 1 unit to leave room for the left wall.
        let right_by_1 = (1, 0);
        for (position, color) in self.play_field.space() {
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
        let text_x = self.play_field.width() + 4;

        ui.draw_text(Position::new(text_x, 1), &format!("Score: {}", self.score));
        ui.draw_text(
            Position::new(text_x, 2),
            &format!("Level: {}", self.level()),
        );
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
            ui.draw_text(
                Position::new(text_x, 12),
                &format!("Loop count: {}", self.loop_count),
            );
            ui.draw_text(
                Position::new(text_x, 13),
                &format!("Fall pace: {}", self.fall_pace()),
            );
        }
    }

    fn end_loop(&self) -> Option<StateName> {
        if self.is_game_over && self.is_restarted {
            log::info!("Transitioning state: Ongoing to Intro");
            Some(StateName::Intro)
        } else {
            None
        }
    }
}
