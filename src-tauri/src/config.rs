use crate::snippet::Snippet;
use crate::snippet_settings::SnippetSettings;
use rusqlite::{params, OptionalExtension, Connection, Result as SqlResult};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self, String> {
        let path = Self::get_db_path();
        let conn = Connection::open(&path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let db = Database {
            conn: Mutex::new(conn),
        };

        db.init_tables()?;
        Ok(db)
    }

    fn get_db_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .expect("Failed to get config directory")
            .join("lemmeDoIt");
        println!("Database will be stored at: {:?}", config_dir);

        std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        config_dir.join("snippets.db")
    }

    fn init_tables(&self) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        println!("Making snipprt table");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                id TEXT PRIMARY KEY,
                trigger TEXT NOT NULL UNIQUE,
                content TEXT NOT NULL,
                description TEXT,
                tags TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                usage_count INTEGER NOT NULL DEFAULT 0,
                is_html BOOLEAN NOT NULL DEFAULT FALSE
            )",
            [],
        )
        .map_err(|e| format!("Failed to create table: {}", e))?;

        // Create index on trigger for faster lookups
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_trigger ON snippets(trigger)",
            [],
        )
        .map_err(|e| format!("Failed to create index: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippet_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            time_delay_ms INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .map_err(|e| format!("Failed to create table: {}", e))?;

        // Ensure there's always one row in snippet_settings
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        conn.execute(
            "INSERT OR IGNORE INTO snippet_settings (id, time_delay_ms, created_at, updated_at) VALUES (1, 200, ?1, ?2)",
            [&timestamp, &timestamp],
        )
        .map_err(|e| format!("Failed to initialize snippet_settings: {}", e))?;

        Ok(())
    }

    pub fn get_snippet_settings(&self) -> Result<SnippetSettings, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT time_delay_ms, created_at, updated_at FROM snippet_settings WHERE id = 1")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;
        let snippet_settings = stmt
            .query_row([],|row| {
                Ok(SnippetSettings {
                    id: "1".to_string(),
                    time_delay_ms: row.get(0)?,
                    created_at: row.get(1)?,
                    updated_at: row.get(2)?,
                })
            })
            .optional()
            .map_err(|e| format!("Failed to query snippet: {}", e))?;

        snippet_settings.ok_or_else(|| "Snippet settings not found".to_string())
    }

    pub fn update_snippet_settings(&self, time_delay_ms: u64) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        conn.execute(
            "UPDATE snippet_settings SET time_delay_ms = ?1, updated_at = ?2 WHERE id = 1",
            params![&time_delay_ms, &timestamp],
        )
        .map_err(|e| format!("Failed to update snippet: {}", e))?;

        Ok(())
    }

    pub fn get_all_snippets(&self) -> Result<Vec<Snippet>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare("SELECT id, trigger, content, description, tags, created_at, updated_at, usage_count, is_html FROM snippets ORDER BY updated_at DESC")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let snippets = stmt
            .query_map([], |row| {
                let tags_str: String = row.get(4)?;
                let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();

                Ok(Snippet {
                    id: row.get(0)?,
                    trigger: row.get(1)?,
                    content: row.get(2)?,
                    description: row.get(3)?,
                    tags,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                    usage_count: row.get(7)?,
                    is_html: row.get(8)?,
                })
            })
            .map_err(|e| format!("Failed to query snippets: {}", e))?
            .collect::<SqlResult<Vec<Snippet>>>()
            .map_err(|e| format!("Failed to collect snippets: {}", e))?;

        Ok(snippets)
    }

    pub fn get_snippet_by_id(&self, id: &str) -> Result<Option<Snippet>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare("SELECT id, trigger, content, description, tags, created_at, updated_at, usage_count, is_html FROM snippets WHERE id = ?1")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let snippet = stmt
            .query_row([id], |row| {
                let tags_str: String = row.get(4)?;
                let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();

                Ok(Snippet {
                    id: row.get(0)?,
                    trigger: row.get(1)?,
                    content: row.get(2)?,
                    description: row.get(3)?,
                    tags,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                    usage_count: row.get(7)?,
                    is_html: row.get(8)?,
                })
            })
            .optional()
            .map_err(|e| format!("Failed to query snippet: {}", e))?;

        Ok(snippet)
    }

    pub fn create_snippet(&self, snippet: &Snippet) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        // Check for duplicate trigger
        let exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM snippets WHERE trigger = ?1)",
                [&snippet.trigger],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to check for duplicate: {}", e))?;

        if exists {
            return Err("Snippet with this trigger already exists".to_string());
        }

        let tags_json = serde_json::to_string(&snippet.tags)
            .map_err(|e| format!("Failed to serialize tags: {}", e))?;

        println!("1234");
        conn.execute(
            "INSERT INTO snippets (id, trigger, content, description, tags, created_at, updated_at, usage_count, is_html) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &snippet.id,
                &snippet.trigger,
                &snippet.content,
                &snippet.description,
                &tags_json,
                snippet.created_at,
                snippet.updated_at,
                snippet.usage_count,
                snippet.is_html
            ],
        )
        .map_err(|e| format!("Failed to insert snippet: {}", e))?;

        println!("5678");
        Ok(())
    }

    pub fn update_snippet(
        &self,
        id: &str,
        trigger: String,
        content: String,
        description: Option<String>,
        tags: Vec<String>,
        is_html: bool,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        // Check for duplicate trigger (excluding current snippet)
        let exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM snippets WHERE trigger = ?1 AND id != ?2)",
                params![&trigger, id],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to check for duplicate: {}", e))?;

        if exists {
            return Err("Another snippet with this trigger already exists".to_string());
        }

        let tags_json = serde_json::to_string(&tags)
            .map_err(|e| format!("Failed to serialize tags: {}", e))?;

        let updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        conn.execute(
            "UPDATE snippets SET trigger = ?1, content = ?2, description = ?3, tags = ?4, updated_at = ?5, is_html = ?7 WHERE id = ?6",
            params![&trigger, &content, &description, &tags_json, updated_at, id, is_html],
        )
        .map_err(|e| format!("Failed to update snippet: {}", e))?;

        Ok(())
    }

    pub fn delete_snippet(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let rows_affected = conn
            .execute("DELETE FROM snippets WHERE id = ?1", [id])
            .map_err(|e| format!("Failed to delete snippet: {}", e))?;

        if rows_affected == 0 {
            return Err("Snippet not found".to_string());
        }

        Ok(())
    }

    pub fn search_snippets(&self, query: &str) -> Result<Vec<Snippet>, String> {
        if query.is_empty() {
            return self.get_all_snippets();
        }

        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let search_pattern = format!("%{}%", query);

        let mut stmt = conn
            .prepare(
                "SELECT id, trigger, content, description, tags, created_at, updated_at, usage_count, is_html
                 FROM snippets 
                 WHERE trigger LIKE ?1 
                    OR content LIKE ?1 
                    OR description LIKE ?1 
                    OR tags LIKE ?1 
                 ORDER BY updated_at DESC"
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let snippets = stmt
            .query_map([&search_pattern], |row| {
                let tags_str: String = row.get(4)?;
                let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();

                Ok(Snippet {
                    id: row.get(0)?,
                    trigger: row.get(1)?,
                    content: row.get(2)?,
                    description: row.get(3)?,
                    tags,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                    usage_count: row.get(7)?,
                    is_html: row.get(8)?,
                })
            })
            .map_err(|e| format!("Failed to query snippets: {}", e))?
            .collect::<SqlResult<Vec<Snippet>>>()
            .map_err(|e| format!("Failed to collect snippets: {}", e))?;

        Ok(snippets)
    }

    pub fn export_snippets(&self) -> Result<String, String> {
        let snippets = self.get_all_snippets()?;
        serde_json::to_string_pretty(&snippets)
            .map_err(|e| format!("Failed to export snippets: {}", e))
    }

    pub fn import_snippets(&self, json: &str) -> Result<usize, String> {
        let snippets: Vec<Snippet> = serde_json::from_str(json)
            .map_err(|e| format!("Invalid JSON format: {}", e))?;

        let mut imported_count = 0;

        for snippet in snippets {
            // Try to insert, skip if trigger already exists
            if self.create_snippet(&snippet).is_ok() {
                imported_count += 1;
            }
        }

        Ok(imported_count)
    }

pub fn increment_usage(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            "UPDATE snippets SET usage_count = usage_count + 1, updated_at = ?1 WHERE id = ?2",
            params![
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                id
            ],
        )
        .map_err(|e| format!("Failed to increment usage: {}", e))?;

        Ok(())
    }
}