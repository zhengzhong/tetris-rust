use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TetrisSettings {
    pub play_field_width: u8,
    pub play_field_height: u8,
    pub enable_cheating: bool,
}

impl Default for TetrisSettings {
    fn default() -> Self {
        Self {
            play_field_width: 10,
            play_field_height: 20,
            enable_cheating: true,
        }
    }
}
