use std::collections::HashMap;

use super::common::{Color, Position};
use super::tetromino::GameWorld;

pub struct PlayField {
    width: u8,
    height: u8,
    space: HashMap<Position, Color>,
}

impl PlayField {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            space: HashMap::new(),
        }
    }

    pub fn width(&self) -> i16 {
        self.width as i16
    }

    pub fn height(&self) -> i16 {
        self.height as i16
    }

    pub fn space(&self) -> &HashMap<Position, Color> {
        &self.space
    }

    pub fn fill_space(&mut self, positions: &[Position], color: Color) {
        for position in positions {
            let old_color = self.space.insert(*position, color);
            if let Some(old_color) = old_color {
                log::error!(
                    "Position {} is already taken (with color: {})",
                    position,
                    old_color
                );
            }
        }
    }

    pub fn destroy_completed_rows(&mut self) -> i16 {
        let rows_completed: Vec<i16> = (0..self.height())
            .filter(|&row| {
                let n_filled = self
                    .space
                    .keys()
                    .filter(|&pos| {
                        let (_, y) = pos.xy();
                        y == row
                    })
                    .count() as i16;
                n_filled == self.width()
            })
            .collect();
        self.destroy_rows(&rows_completed);
        rows_completed.len() as i16
    }

    pub fn fade_to_gray(&mut self) {
        self.space
            .values_mut()
            .for_each(|color| *color = Color::Gray);
    }

    pub fn clear(&mut self) {
        self.space.clear();
    }

    pub fn destroy_rows(&mut self, rows: &[i16]) {
        if rows.is_empty() {
            return;
        }

        let new_space: HashMap<Position, Color> = self
            .space
            .iter()
            // Remove those that are to be destroyed.
            .filter(|(pos, _)| {
                let (_, y) = pos.xy();
                !rows.contains(&y)
            })
            // Push those downwards if one or more rows are destroyed below them.
            .map(|(pos, color)| {
                let (x, y) = pos.xy();
                let n_rows_to_fall = rows.iter().filter(|&row| row > &y).count();
                let new_pos = Position::new(x, y + n_rows_to_fall as i16);
                (new_pos, *color)
            })
            .collect();
        self.space = new_space;
    }
}

impl GameWorld for PlayField {
    fn is_free(&self, positions: &[crate::common::Position]) -> bool {
        positions.iter().all(|position| {
            let (x, y) = position.xy();
            (0..self.width()).contains(&x) && (0..self.height()).contains(&y) // not out of bound
            && !self.space.contains_key(position) // not occupied
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destroy_completed_rows() {
        let mut field = PlayField::new(10, 20);
        let n = field.height() - 6;

        let get_row = |xs: Vec<i16>, y: i16| -> Vec<Position> {
            xs.into_iter().map(|x| Position::new(x, y)).collect()
        };
        let get_complete_row =
            |y: i16| -> Vec<Position> { get_row((0..field.width()).collect(), y) };

        // row N + 0: not complete
        // row N + 1: complete
        // row N + 2: complete
        // row N + 3: incomplete
        // row N + 4: complete
        // row N + 5: incomplete
        let positions_at_n0 = get_row(vec![1, 2, 4], n);
        let positions_at_n1 = get_complete_row(n + 1);
        let positions_at_n2 = get_complete_row(n + 2);
        let positions_at_n3 = get_row(vec![3], n + 3);
        let positions_at_n4 = get_complete_row(n + 4);
        let positions_at_n5 = get_row(vec![6, 7], n + 5);
        field.fill_space(&positions_at_n0, Color::Teal);
        field.fill_space(&positions_at_n1, Color::Yellow);
        field.fill_space(&positions_at_n2, Color::Purple);
        field.fill_space(&positions_at_n3, Color::Blue);
        field.fill_space(&positions_at_n4, Color::Orange);
        field.fill_space(&positions_at_n5, Color::Green);

        field.destroy_completed_rows();

        // row N + 0 falls by +3 rows
        // row N + 3 falls by +1 rows
        // row N + 5 stays still
        let fall_by_rows = |n_rows: i16| (0, n_rows);
        let expected_space: HashMap<Position, Color> = [
            positions_at_n0
                .iter()
                .map(|pos| (pos.updated(fall_by_rows(3)), Color::Teal))
                .collect::<Vec<_>>(),
            positions_at_n3
                .iter()
                .map(|pos| (pos.updated(fall_by_rows(1)), Color::Blue))
                .collect::<Vec<_>>(),
            positions_at_n5
                .iter()
                .map(|pos| (pos.updated(fall_by_rows(0)), Color::Green))
                .collect::<Vec<_>>(),
        ]
        .concat()
        .into_iter()
        .collect();
        assert_eq!(field.space(), &expected_space);
    }

    #[test]
    fn fade_to_gray() {
        let mut field = PlayField::new(10, 20);
        let positions = vec![
            Position::new(0, 0),
            Position::new(1, 1),
            Position::new(2, 2),
        ];
        field.fill_space(&positions, Color::Teal);

        field.fade_to_gray();

        let expected_space: HashMap<Position, Color> =
            positions.iter().map(|pos| (*pos, Color::Gray)).collect();
        assert_eq!(field.space(), &expected_space);
    }
}
