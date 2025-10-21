use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetSettings {
    pub id: String,
    pub keyboard_trigger_key: String,
    pub time_delay_ms: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl SnippetSettings {
    pub fn new(id: String, keyboard_trigger_key: State, time_delay_ms: u64, created_at: u64, updated_at: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            keyboard_trigger_key,
            time_delay_ms,
            created_at: timestamp,
            updated_at: timestamp
        }
    }

    pub fn update(&mut self, id: String, keyboard_trigger_key: State, time_delay_ms: u64, created_at: u64, updated_at: u64) {
        self.keyboard_trigger_key = keyboard_trigger_key;
        self.time_delay_ms = time_delay_ms;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}