use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TetrisSettings {
    pub play_field_width: u8,
    pub play_field_height: u8,
    pub scores_for_rows_destroyed: [u32; 4],
    pub score_per_level: u32,
    pub fall_pace_slowest: u8,
    pub fall_pace_fastest: u8,
    pub enable_cheating: bool,
}

impl Default for TetrisSettings {
    fn default() -> Self {
        Self {
            play_field_width: 10,
            play_field_height: 20,
            scores_for_rows_destroyed: [10, 30, 50, 100],
            score_per_level: 200,
            fall_pace_slowest: 20,
            fall_pace_fastest: 3,
            enable_cheating: true,
        }
    }
}
