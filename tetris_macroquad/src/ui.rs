use std::collections::HashMap;
use macroquad::prelude::*;

use tetris_game::{Button, Color as TetrisColor, GamePad, GameUI, Position};

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

    pub fn refresh_input(&mut self) {
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
                self.pressed.insert(button, is_key_pressed(key));
            });

        match get_char_pressed() {
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
        *self.pressed.get(&button).unwrap_or(&false)
    }

    fn cheat_code(&self) -> Option<char> {
        self.cheat_code
    }
}

pub struct MacroquadUI;

impl MacroquadUI {
    const BRICK_SIZE: f32 = 40.0;
    const BRICK_SHADOW: f32 = 3.0;
    const FONT_SIZE: f32 = 32.0;

    pub fn new() -> Self {
        Self
    }

    pub fn clear_background(&self) {
        clear_background(BLACK);
    }

    fn to_screen_x(&self, x: i16) -> f32 {
        f32::from(x) * Self::BRICK_SIZE
    }

    fn to_screen_y(&self, y: i16) -> f32 {
        f32::from(y) * Self::BRICK_SIZE
    }

    fn to_screen_xy(&self, (x, y): (i16, i16)) -> (f32, f32) {
        (self.to_screen_x(x), self.to_screen_y(y))
    }
}

impl GameUI for MacroquadUI {
    fn draw_background(&mut self) {
    }

    fn draw_debugging_grids(&mut self) {
        let screen_x_max = screen_width();
        let screen_y_max = screen_height();
        let n_cols = (screen_x_max / Self::BRICK_SIZE).floor() as i16;
        let n_rows = (screen_y_max / Self::BRICK_SIZE).floor() as i16;
        // Draw vertical lines.
        for x in 0..=n_cols {
            let screen_x = self.to_screen_x(x);
            draw_line(screen_x, 0.0, screen_x, screen_y_max, 1.0, GRAY);
        }
        // Draw horizontal lines.
        for y in 0..=n_rows {
            let screen_y = self.to_screen_y(y);
            draw_line(0.0, screen_y, screen_x_max, screen_y, 1.0, GRAY);
        }
    }

    /// Draw a brick on the left-side panel.
    /// Size of the left-side panel is 10x20 bricks.
    fn draw_brick(&mut self, pos: Position, color: TetrisColor) {
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
            Self::BRICK_SIZE, Self::BRICK_SHADOW,
            darken(&color),
        );
        draw_rectangle(
            screen_x, screen_y,
            Self::BRICK_SHADOW, Self::BRICK_SIZE,
            darken(&color),
        );
        draw_rectangle(
            screen_x + Self::BRICK_SIZE - Self::BRICK_SHADOW, screen_y + Self::BRICK_SHADOW,
            Self::BRICK_SHADOW, Self::BRICK_SIZE - Self::BRICK_SHADOW,
            lighten(&color),
        );
        draw_rectangle(
            screen_x, screen_y + Self::BRICK_SIZE - Self::BRICK_SHADOW,
            Self::BRICK_SIZE, Self::BRICK_SHADOW,
            lighten(&color),
        );
    }

    fn draw_text(&mut self, pos: Position, msg: &str) {
        let (screen_x, screen_y) = self.to_screen_xy(pos.xy());
        draw_text(msg, screen_x, screen_y, Self::FONT_SIZE, WHITE);
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
