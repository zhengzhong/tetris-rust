use std::collections::HashMap;
use macroquad::prelude::*;

use crate::common::{Button, Color as TetrisColor, GamePad, GameUI, Position};

pub struct MacroquadGamePad {
    pressed: HashMap<Button, bool>,
    cheat_code: Option<char>,
}

impl MacroquadGamePad {
    pub fn new() -> Self {
        Self {
            pressed: HashMap::new(),
            cheat_code: None,
        }
    }

    pub fn set_pressed(&mut self, button: Button, pressed: bool) {
        self.pressed.insert(button, pressed);
    }

    pub fn set_cheat_code(&mut self, cheat_code: Option<char>) {
        match cheat_code {
            Some(ch) if ch >= 'a' && ch <= 'z' => self.cheat_code = Some(ch),
            _ => self.cheat_code = None,
        }
    }
}

impl GamePad for MacroquadGamePad {
    fn direction(&self) -> (i16, i16) {
        let mut offset_x = 0;
        let mut offset_y = 0;
        if self.is_pressed(Button::Left) {
            offset_x -= 1;
        }
        if self.is_pressed(Button::Right) {
            offset_x += 1;
        }
        if self.is_pressed(Button::Up) {
            offset_y -= 1;
        }
        if self.is_pressed(Button::Down) {
            offset_y += 1;
        }
        (offset_x, offset_y)
    }

    fn is_pressed(&self, button: Button) -> bool {
        self.pressed.get(&button).unwrap_or_else(|| &false).clone()
    }

    fn cheat_code(&self) -> Option<char> {
        self.cheat_code.clone()
    }
}

pub struct MacroquadUI;

impl MacroquadUI {
    const GRID_WIDTH: i16 = 10; // TODO: Also defined in `Tetris`
    const GRID_HEIGHT: i16 = 20; // TODO: Also defined in `Tetris`

    // TODO: Use a `GameConf`
    const BRICK_SIZE: f32 = 40.0;
    const BRICK_ZERO_X: f32 = Self::BRICK_SIZE;
    const BRICK_ZERO_Y: f32 = Self::BRICK_SIZE;

    const SHADOW_THICKNESS: f32 = 3.0;

    const FONT_SIZE: f32 = 32.0;

    pub fn new() -> Self {
        Self
    }

    fn to_screen_x(&self, x: i16) -> f32 {
        Self::BRICK_ZERO_X + f32::from(x) * Self::BRICK_SIZE
    }

    fn to_screen_y(&self, y: i16) -> f32 {
        Self::BRICK_ZERO_Y + f32::from(y) * Self::BRICK_SIZE
    }

    fn to_screen_xy(&self, (x, y): (i16, i16)) -> (f32, f32) {
        (self.to_screen_x(x), self.to_screen_y(y))
    }

    fn to_text_xy(&self, y: i16) -> (f32, f32) {
        // Count 3 extra bricks: 2 the left and right wall and 1 for the text margin.
        let x = self.to_screen_x(Self::GRID_WIDTH + 3);
        let y = self.to_screen_y(y);
        (x, y)
    }
}

impl GameUI for MacroquadUI {
    fn draw_background(&mut self) {
        let wall_color = TetrisColor::Gray;
        for y in -1..=Self::GRID_HEIGHT {
            self.draw_brick(&Position::new(-1, y), wall_color);
            self.draw_brick(&Position::new(Self::GRID_WIDTH, y), wall_color);
        }
        for x in -1..=Self::GRID_WIDTH {
            self.draw_brick(&Position::new(x, Self::GRID_HEIGHT), wall_color);
        }
    }
    
    fn draw_foreground(&mut self) {
    }

    fn draw_grids(&mut self) {
        let (screen_x_min, screen_y_min) = self.to_screen_xy((0, 0));
        let (screen_x_max, screen_y_max) = self.to_screen_xy(
            (Self::GRID_WIDTH, Self::GRID_HEIGHT)
        );
        for x in 0..(Self::GRID_WIDTH + 1) {
            let screen_x = self.to_screen_x(x);
            draw_line(
                screen_x, screen_y_min,
                screen_x, screen_y_max,
                1.0, GRAY,
            );
        }
        for y in 0..(Self::GRID_HEIGHT + 1) {
            let screen_y = self.to_screen_y(y);
            draw_line(
                screen_x_min, screen_y,
                screen_x_max, screen_y,
                1.0, GRAY,
            );
        }
    }

    /// Draw a brick on the left-side panel.
    /// Size of the left-side panel is 10x20 bricks.
    fn draw_brick(&mut self, pos: &Position, color: TetrisColor) {
        let (screen_x, screen_y) = self.to_screen_xy(pos.xy());
        let color = to_color(color);
        draw_rectangle(
            screen_x, screen_y,
            Self::BRICK_SIZE, Self::BRICK_SIZE,
            color,
        );

        // Draw shadows.
        draw_rectangle(
            screen_x, screen_y,
            Self::BRICK_SIZE, Self::SHADOW_THICKNESS,
            darken(&color),
        );
        draw_rectangle(
            screen_x, screen_y,
            Self::SHADOW_THICKNESS, Self::BRICK_SIZE,
            darken(&color),
        );
        draw_rectangle(
            screen_x + Self::BRICK_SIZE - Self::SHADOW_THICKNESS, screen_y + Self::SHADOW_THICKNESS,
            Self::SHADOW_THICKNESS, Self::BRICK_SIZE - Self::SHADOW_THICKNESS,
            lighten(&color),
        );
        draw_rectangle(
            screen_x, screen_y + Self::BRICK_SIZE - Self::SHADOW_THICKNESS,
            Self::BRICK_SIZE, Self::SHADOW_THICKNESS,
            lighten(&color),
        );
    }

    fn draw_text(&mut self, row: i16, msg: &str) {
        let (text_x, text_y) = self.to_text_xy(row);
        draw_text(msg, text_x, text_y, Self::FONT_SIZE, WHITE);
    }
}

// region: ---------- Utilities -------------------------------------------------------------------

fn to_color(color: TetrisColor) -> Color {
    match color {
        TetrisColor::Teal => SKYBLUE,
        TetrisColor::Yellow => YELLOW,
        TetrisColor::Purple => PURPLE,
        TetrisColor::Blue => BLUE,
        TetrisColor::Orange => ORANGE,
        TetrisColor::Green => GREEN,
        TetrisColor::Red => RED,
        TetrisColor::Gray => GRAY,
    }
}

fn darken(color: &Color) -> Color {
    Color {
        r: color.r * 0.75,
        g: color.g * 0.75,
        b: color.b * 0.75,
        a: 1.0,
     }
}

fn lighten(color: &Color) -> Color {
    Color {
        r: 1.0_f32.min(color.r * 1.25),
        g: 1.0_f32.min(color.g * 1.25),
        b: 1.0_f32.min(color.b * 1.25),
        a: 1.0,
     }
}

// endregion
