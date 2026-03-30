use serde::{Deserialize, Serialize};
use crate::constants::SAVE_FILE_NAME;

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    pub high_score: u32,
    #[serde(default = "default_volume")]
    pub master_volume: f32,
    #[serde(default = "default_music_volume")]
    pub music_volume: f32,
    #[serde(default = "default_sfx_volume")]
    pub sfx_volume: f32,
    #[serde(default = "default_difficulty")]
    pub difficulty: u8, // 0=Easy, 1=Normal, 2=Hard
}

fn default_difficulty() -> u8 { 1 }
fn default_volume() -> f32 { 0.8 }
fn default_music_volume() -> f32 { 0.7 }
fn default_sfx_volume() -> f32 { 0.8 }

pub fn load_save() -> SaveData {
    std::fs::read_to_string(SAVE_FILE_NAME)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn write_save(data: &SaveData) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = std::fs::write(SAVE_FILE_NAME, json);
    }
}
