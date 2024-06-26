use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i16,
    y: i16,
}

impl Position {
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn xy(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    pub fn updated(&self, (dx, dy): (i16, i16)) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum Color {
    Teal,
    Yellow,
    Purple,
    Blue,
    Orange,
    Green,
    Red,
    Gray,
}

impl Copy for Color {}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Button {
    Left,
    Right,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

pub trait GamePad {
    fn direction(&self) -> (i16, i16);
    fn is_pressed(&self, button: Button) -> bool;
    fn cheat_code(&self) -> Option<char>;
}

/// Allow to draw onto the game UI.
pub trait GameUI {
    fn draw_background(&mut self);
    fn draw_brick(&mut self, pos: Position, color: Color);
    fn draw_text(&mut self, pos: Position, msg: &str);
    fn draw_debugging_grids(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_color_names {
        ( $( $color:expr ),* ) => {
            $( assert_eq!(format!("Color::{}", $color), stringify!($color)); )*
        };
    }

    #[test]
    fn color_names() {
        test_color_names!(
            Color::Teal,
            Color::Yellow,
            Color::Purple,
            Color::Blue,
            Color::Orange,
            Color::Green,
            Color::Red,
            Color::Gray
        );
    }
}
