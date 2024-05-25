use macroquad::prelude::*;
use std::collections::HashMap;

use tetris_game::{Button, Color as TetrisColor, GamePad, GameUI, Position};

use crate::conf::{GamePadSettings, UISettings};

pub struct MacroquadGamePad {
    pressed: HashMap<Button, bool>,
    cheat_code: Option<char>,
}

impl MacroquadGamePad {
    const KEY_CODE_MAPPINGS: [(KeyCode, Button); 8] = [
        (KeyCode::Left, Button::Left),
        (KeyCode::Right, Button::Right),
        (KeyCode::Up, Button::Up),
        (KeyCode::Down, Button::Down),
        (KeyCode::Space, Button::A),
        (KeyCode::Tab, Button::B),
        (KeyCode::LeftControl, Button::Select),
        (KeyCode::Enter, Button::Start),
    ];

    pub fn new(_settings: &GamePadSettings) -> Self {
        Self {
            pressed: HashMap::new(),
            cheat_code: None,
        }
    }

    pub fn refresh_input(&mut self) {
        Self::KEY_CODE_MAPPINGS
            .iter()
            .for_each(|(key_code, button)| {
                self.pressed.insert(*button, is_key_pressed(*key_code));
            });

        match get_char_pressed() {
            Some(ch) if ch.is_ascii_alphanumeric() => self.cheat_code = Some(ch),
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

pub struct MacroquadUI<'a> {
    settings: &'a UISettings,
}

impl<'a> MacroquadUI<'a> {
    pub fn new(settings: &'a UISettings) -> Self {
        Self { settings }
    }

    pub fn brick_size(&self) -> f32 {
        f32::from(self.settings.brick_size)
    }

    pub fn brick_shadow(&self) -> f32 {
        f32::from(self.settings.brick_shadow)
    }

    pub fn font_size(&self) -> f32 {
        f32::from(self.settings.font_size)
    }

    pub fn clear_background(&self) {
        clear_background(BLACK);
    }

    fn to_screen_x(&self, x: i16) -> f32 {
        f32::from(x) * self.brick_size()
    }

    fn to_screen_y(&self, y: i16) -> f32 {
        f32::from(y) * self.brick_size()
    }

    fn to_screen_xy(&self, (x, y): (i16, i16)) -> (f32, f32) {
        (self.to_screen_x(x), self.to_screen_y(y))
    }
}

impl<'a> GameUI for MacroquadUI<'a> {
    fn draw_background(&mut self) {}

    fn draw_debugging_grids(&mut self) {
        let screen_x_max = screen_width();
        let screen_y_max = screen_height();
        let n_cols = (screen_x_max / self.brick_size()).floor() as i16;
        let n_rows = (screen_y_max / self.brick_size()).floor() as i16;
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
            screen_x,
            screen_y,
            self.brick_size(),
            self.brick_size(),
            color,
        );

        // Draw shadows.
        draw_rectangle(
            screen_x,
            screen_y,
            self.brick_size(),
            self.brick_shadow(),
            darken(&color),
        );
        draw_rectangle(
            screen_x,
            screen_y,
            self.brick_shadow(),
            self.brick_size(),
            darken(&color),
        );
        draw_rectangle(
            screen_x + self.brick_size() - self.brick_shadow(),
            screen_y + self.brick_shadow(),
            self.brick_shadow(),
            self.brick_size() - self.brick_shadow(),
            lighten(&color),
        );
        draw_rectangle(
            screen_x,
            screen_y + self.brick_size() - self.brick_shadow(),
            self.brick_size(),
            self.brick_shadow(),
            lighten(&color),
        );
    }

    fn draw_text(&mut self, pos: Position, msg: &str) {
        let (screen_x, screen_y) = self.to_screen_xy(pos.xy());
        draw_text(msg, screen_x, screen_y, self.font_size(), WHITE);
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
