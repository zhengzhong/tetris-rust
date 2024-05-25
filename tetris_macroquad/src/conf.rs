use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use tetris_game::TetrisSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub tetris: TetrisSettings,
    pub ui: UISettings,
    pub game_pad: GamePadSettings,

    /// Interval in millis between game loops.
    pub loop_interval_millis: u8,
}

impl Settings {
    /// Compute the window size as (width, height) given the block size.
    /// The window will hold 2 play fields, including the surrounding
    /// three-sided wall (without the upper side) for each play field which
    /// has a width of one block size.
    pub fn window_size(&self) -> (i32, i32) {
        let brick_size = self.ui.brick_size as i32;
        // Left side and right side wall for each play field; 2 play fields.
        let n_cols = (self.tetris.play_field_width as i32 + 2) * 2;
        // Bottom side wall for the play fields.
        let n_rows = self.tetris.play_field_height as i32 + 1;
        (n_cols * brick_size, n_rows * brick_size)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tetris: TetrisSettings::default(),
            ui: UISettings::default(),
            game_pad: GamePadSettings::default(),
            loop_interval_millis: 25,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UISettings {
    pub brick_size: u8,
    pub brick_shadow: u8,
    pub font_size: u8,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            brick_size: 40,
            brick_shadow: 3,
            font_size: 32,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamePadSettings {
    pub dummy: u8,
}

impl Default for GamePadSettings {
    fn default() -> Self {
        Self { dummy: 42 }
    }
}

pub fn load_settings() -> Result<Settings, ConfigError> {
    let defaults = Config::try_from(&Settings::default()).expect("Fail to get default settings");
    let work_dir = std::env::current_dir().expect("Fail to get current dir");
    let settings = Config::builder()
        .add_source(defaults)
        .add_source(File::from(work_dir.join("tetris")).required(false))
        .add_source(Environment::with_prefix("TETRIS").separator("__"))
        .build()?;
    settings.try_deserialize()
}
