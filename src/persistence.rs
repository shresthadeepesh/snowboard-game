use serde::{Deserialize, Serialize};
use crate::constants::SAVE_FILE_NAME;

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    pub high_score: u32,
}

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
