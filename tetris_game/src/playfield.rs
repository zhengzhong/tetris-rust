use std::collections::HashMap;

use super::common::{Color, Position};
use super::tetromino::GameWorld;

pub struct PlayField {
    spaces: HashMap<Position, Color>,
}

impl PlayField {
    pub const WIDTH: i16 = 10;
    pub const HEIGHT: i16 = 20;

    pub fn new() -> Self {
        Self {
            spaces: HashMap::new(),
        }
    }

    pub fn spaces(&self) -> &HashMap<Position, Color> {
        &self.spaces
    }

    pub fn fill_spaces(&mut self, positions: &[Position], color: Color) {
        for position in positions {
            let old_color = self.spaces.insert(position.clone(), color);
            if let Some(old_color) = old_color {
                println!("[WARN] Position {} already had color {}", position, old_color);
            }
        }
    }

    pub fn destroy_completed_rows(&mut self) -> i16 {
        let rows_completed: Vec<i16> = (0..Self::HEIGHT).into_iter()
            .filter(|&row| {
                let n_filled = self.spaces.keys()
                    .filter(|&pos| {
                        let (_, y) = pos.xy();
                        y == row
                    })
                    .count() as i16;
                n_filled == Self::WIDTH
            })
            .collect();
        self.destroy_rows(&rows_completed);
        rows_completed.len() as i16
    }

    pub fn fade_to_gray(&mut self) {
        self.spaces.values_mut().for_each(|color| *color = Color::Gray);
    }

    pub fn clear(&mut self) {
        self.spaces.clear();
    }

    pub fn destroy_rows(&mut self, rows: &[i16]) {
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

impl GameWorld for PlayField {
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
