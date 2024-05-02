use std::collections::HashMap;
use std::mem;

use rand::random;

use crate::common::{Button, Color, GamePad, GameUI, GameWorld, Position};
use crate::tetromino::{Shape, Tetromino};

// region: TetrisHeap -----------------------------------------------------------------------------

struct TetrisHeap {
    spaces: HashMap<Position, Color>,
}

impl TetrisHeap {
    const WIDTH: i16 = 10;
    const HEIGHT: i16 = 20;

    fn new() -> Self {
        Self {
            spaces: HashMap::new(),
        }
    }

    fn fill_spaces(&mut self, positions: &[Position], color: Color) {
        for position in positions {
            let old_color = self.spaces.insert(position.clone(), color);
            if let Some(old_color) = old_color {
                println!("[WARN] Position {} already had color {}", position, old_color);
            }
        }
    }

    fn destroy_completed_rows(&mut self) -> u32 {
        // TODO: Make it more functional!
        // TODO: Make types of `i16` and `usize` play nicer!
        let mut rows_to_destroy: Vec<i16> = Vec::new();
        for y in (0..Self::HEIGHT).rev() {
            let n_spaces_filled = self.spaces.keys().filter(|&pos| pos.xy().1 == y).count();
            if n_spaces_filled == Self::WIDTH as usize {
                rows_to_destroy.push(y);
            } else if n_spaces_filled == 0 {
                break;
            }
        }
        self.destroy_rows(&rows_to_destroy);
        rows_to_destroy.len() as u32
    }

    fn fade_to_gray(&mut self) {
        self.spaces.values_mut().for_each(|color| *color = Color::Gray);
    }

    fn clear(&mut self) {
        self.spaces.clear();
    }

    fn destroy_rows(&mut self, rows: &[i16]) {
        if rows.is_empty() {
            return;
        }

        let new_spaces: HashMap<Position, Color> = self.spaces
            .iter()
            // Remove those that are to be destroyed.
            .filter(|(pos, _)| {
                let (_, y) = pos.xy();
                !rows.contains(&y)
            })
            // Push those downwards if one or more rows are destroyed below them.
            .map(|(pos, color)| {
                let (x, y) = pos.xy();
                let n_rows_to_fall = rows.iter()
                    .filter(|&row| row > &y)
                    .count();
                let new_pos = Position::new(x, y + n_rows_to_fall as i16);
                (new_pos, color.clone())
            })
            .collect();
        self.spaces = new_spaces;
    }
}

impl GameWorld for TetrisHeap {
    fn is_free(&self, positions: &[crate::common::Position]) -> bool {
        for position in positions {
            // Check if the position is out of the bounds.
            let (x, y) = position.xy();
            if x < 0 || x >= Self::WIDTH || y < 0 || y >= Self::HEIGHT {
                return false;
            }
            // Check if the position is taken.
            if self.spaces.contains_key(position) {
                return false;
            }
        }
        true
    }
}

// endregion

// region: Tetris ---------------------------------------------------------------------------------

pub struct Tetris {
    is_ongoing: bool, // TODO: Use the "empty struct" pattern to model game state!
    loop_count: i32,
    fall_speed: i32,
    next_tetromino: Option<Tetromino>,
    active_tetromino: Option<Tetromino>,
    heap: TetrisHeap,
    score: u32,
    cheat_codes: String,
    is_game_over: bool,
    is_debug_enabled: bool,
}

impl Tetris {
    const TOP_CENTER_POS: Position = Position::new(TetrisHeap::WIDTH / 2 - 2, 0);

    const MAX_FALL_SPEED: i32 = 20;
    const SCORE_1_ROW_DESTROYED: u32 = 10;
    const SCORE_2_ROWS_DESTROYED: u32 = 50;
    const SCORE_3_ROWS_DESTROYED: u32 = 100;
    const SCORE_4_ROWS_DESTROYED: u32 = 200;

    pub fn new() -> Self {
        Self {
            is_ongoing: false,
            loop_count: 0,
            fall_speed: 1,
            next_tetromino: None,
            active_tetromino: None,
            heap: TetrisHeap::new(),
            score: 0,
            cheat_codes: String::new(),
            is_game_over: false,
            is_debug_enabled: false,
        }
    }

    pub fn start_game(&mut self) {
        self.is_ongoing = true;
        self.loop_count = 0;
        self.fall_speed = 1;
        self.next_tetromino = None;
        self.active_tetromino = None;
        self.heap.clear();
        self.score = 0;
        self.cheat_codes.clear();
        self.is_game_over = false;
    }

    pub fn start_loop(&mut self) {
        if !self.is_ongoing {
            return;
        }
        self.loop_count += 1;
        if self.active_tetromino.is_none() {
            let tetromino = self.take_next_tetromino();
            if self.heap.is_free(tetromino.bricks()) {
                self.active_tetromino = Some(tetromino);
            } else {
                println!("Game is over!");
                self.heap.fade_to_gray();
                self.is_game_over = true;
            }
        }
    }

    pub fn process_input(&mut self, pad: &dyn GamePad) {
        self.process_direction(pad.direction());
        self.process_a(pad.is_pressed(Button::A));
        self.process_b(pad.is_pressed(Button::B));
        self.process_select(pad.is_pressed(Button::Select));
        self.process_start(pad.is_pressed(Button::Start));
        if let Some(cheat_code) = pad.cheat_code() {
            self.process_cheat_code(cheat_code);
        }
    }

    pub fn update_state(&mut self) {
        if !self.is_ongoing {
            return;
        }
        let fall_per_n_loops = self.fall_per_n_loops();
        if self.loop_count % fall_per_n_loops == 0 {
            if let Some(tetromino) = self.active_tetromino.as_mut() {
                let has_fallen_down = tetromino.fall_down(&self.heap);
                if !has_fallen_down {
                    // The tetromino has reached the bottom.
                    self.heap.fill_spaces(tetromino.bricks(), tetromino.color());
                    self.active_tetromino = None;
                    let n_rows_destroyed = self.heap.destroy_completed_rows();
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

    pub fn draw(&self, ui: &mut dyn GameUI) {
        ui.draw_background();

        if self.is_debug_enabled {
            ui.draw_debugging_grids();
        }

        // Draw the wall surrounding the play field.
        let wall_color = Color::Gray;
        for y in 0..=TetrisHeap::HEIGHT {
            ui.draw_brick(&Position::new(0, y), wall_color);
            ui.draw_brick(&Position::new(TetrisHeap::WIDTH + 1, y), wall_color);
        }
        for x in 0..=TetrisHeap::WIDTH {
            ui.draw_brick(&Position::new(x, TetrisHeap::HEIGHT), wall_color);
        }

        // Draw the inactive bricks in the play field and the active tetromino.
        // Note: We move the bricks to the right by 1 unit to leave room for the left wall.
        let right_by_1 = (1, 0);
        for (position, color) in &self.heap.spaces {
            ui.draw_brick(&position.updated(right_by_1), *color);
        }
        if let Some(tetromino) = self.active_tetromino.as_ref() {
            let color = tetromino.color();
            for brick in tetromino.bricks() {
                ui.draw_brick(&brick.updated(right_by_1), color);
            }
        }

        // Texts are shown on the right panel, so leave space for the play field
        // + 2 units for the wall + 2 units for left margin.
        let text_x = TetrisHeap::WIDTH + 4;

        ui.draw_text(&Position::new(text_x, 1), &format!("Score: {}", self.score));
        ui.draw_text(&Position::new(text_x, 2), &format!("Level: {}", self.level()));
        ui.draw_text(&Position::new(text_x, 3), &self.cheat_codes);
        if let Some(next_tetromino) = self.next_tetromino.as_ref() {
            ui.draw_text(&Position::new(text_x, 4), "Next:");
            let aligned_with_text = (text_x, 5);
            let color = next_tetromino.color();
            for brick in next_tetromino.bricks() {
                ui.draw_brick(&brick.updated(aligned_with_text), color);
            }
        }
        if self.is_game_over {
            ui.draw_text(&Position::new(text_x, 9), "Game Over!");
        }

        if self.is_debug_enabled {
            ui.draw_text(&Position::new(text_x, 11), "---- DEBUG ----");
            ui.draw_text(&Position::new(text_x, 12), &format!("Loop count: {}", self.loop_count));
            ui.draw_text(&Position::new(text_x, 13), &format!("Fall speed: {}", self.fall_speed));
        }
    }

    pub fn end_loop(&mut self) {
        if !self.is_ongoing {
            return;
        }
        if self.is_game_over {
            println!("Stopping the game");
            self.is_ongoing = false;
        } else {
            self.fall_speed = (self.level() + 1) as i32;
        }
    }

    // Private

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
            Tetromino::new(Shape::pick(random()), &Position::new(0, 0))
        );
        mem::swap(&mut self.next_tetromino, &mut next_tetromino);
        match next_tetromino {
            Some(tetromino) => Tetromino::new(tetromino.shape(), &Self::TOP_CENTER_POS),
            None => Tetromino::new(Shape::pick(random()), &Self::TOP_CENTER_POS),
        }
    }

    fn process_direction(&mut self, direction: (i16, i16)) {
        if !self.is_ongoing {
            return;
        }
        if let Some(tetromino) = self.active_tetromino.as_mut() {
            tetromino.move_towards(direction, &self.heap);
        }
    }

    fn process_a(&mut self, is_pressed: bool) {
        if !self.is_ongoing {
            return;
        }
        if let Some(tetromino) = self.active_tetromino.as_mut() {
            if is_pressed {
                tetromino.rotate_right(&self.heap);
            }
        }
    }

    fn process_b(&mut self, is_pressed: bool) {
        if !self.is_ongoing {
            return;
        }
        if let Some(tetromino) = self.active_tetromino.as_mut() {
            if is_pressed {
                tetromino.fall_to_bottom(&self.heap);
            }
        }
    }

    fn process_select(&mut self, is_pressed: bool) {
        if is_pressed {
            // Toggle debug mode.
            self.is_debug_enabled = !self.is_debug_enabled;
        }
    }

    fn process_start(&mut self, is_pressed: bool) {
        if !is_pressed {
            return;
        }
        if !self.is_ongoing {
            self.start_game();
        } else {
            // Game is ongoing: Let's cheat!
            if !self.cheat_codes.is_empty() {
                let mut cheat_codes = String::new();
                mem::swap(&mut self.cheat_codes, &mut cheat_codes);
                self.cheat(&cheat_codes);
            }
        }
    }

    fn process_cheat_code(&mut self, cheat_code: char) {
        if self.cheat_codes.len() < 20 {
            self.cheat_codes.push(cheat_code);
        }
    }

    fn cheat(&mut self, cheat_codes: &str) {
        match cheat_codes {
            "solongmarianne" => {
                self.next_tetromino = Some(Tetromino::new(Shape::I, &Position::new(0, 0)));
            },
            "paintitblack" => {
                self.start_game();
            }
            "obladi" => {
                let rows = vec![TetrisHeap::HEIGHT - 1];
                self.heap.destroy_rows(&rows);
            }
            "oblada" => {
                let rows = vec![TetrisHeap::HEIGHT - 1, TetrisHeap::HEIGHT - 2];
                self.heap.destroy_rows(&rows);
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

// endregion
