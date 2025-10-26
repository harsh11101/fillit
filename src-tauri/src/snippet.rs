use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub trigger: String,
    pub content: String,  // Now supports HTML content
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub usage_count: u32,
    pub is_html: bool,  // Flag to indicate if content is HTML
}

impl Snippet {
    pub fn new(trigger: String, content: String, description: Option<String>, tags: Vec<String>, is_html: bool) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            trigger,
            content,
            description,
            tags,
            created_at: timestamp,
            updated_at: timestamp,
            usage_count: 0,
            is_html,
        }
    }

    pub fn _update(&mut self, trigger: String, content: String, description: Option<String>, tags: Vec<String>, is_html: bool) {
        self.trigger = trigger;
        self.content = content;
        self.description = description;
        self.tags = tags;
        self.is_html = is_html;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn _increment_usage(&mut self) {
        self.usage_count += 1;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn _matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.trigger.to_lowercase().contains(&query_lower)
            || self.content.to_lowercase().contains(&query_lower)
            || self.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
            || self.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
    }

    pub fn _get_sanitized_html(&self) -> String {
        if !self.is_html {
            // Escape HTML if not marked as HTML content
            self.content
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;")
        } else {
            // Return as-is for HTML content
            // Note: In production, you'd want to use a proper HTML sanitizer
            self.content.clone()
        }
    }
}