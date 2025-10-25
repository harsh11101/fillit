use crate::config::Database;
use crate::snippet::Snippet;
use crate::snippet_settings::SnippetSettings;
use tauri::State;

#[tauri::command]
pub fn get_all_snippets(db: State<Database>) -> Result<Vec<Snippet>, String> {
    db.get_all_snippets()
}

#[tauri::command]
pub fn get_snippet_by_id(id: String, db: State<Database>) -> Result<Option<Snippet>, String> {
    db.get_snippet_by_id(&id)
}

#[tauri::command]
pub fn create_snippet(
    trigger: String,
    content: String,
    description: Option<String>,
    is_html: bool,
    tags: Vec<String>,
    db: State<Database>,
) -> Result<Snippet, String> {
    let snippet = Snippet::new(trigger, content, description, tags, is_html);
    db.create_snippet(&snippet)?;
    Ok(snippet)
}

#[tauri::command]
pub fn update_snippet(
    id: String,
    trigger: String,
    content: String,
    description: Option<String>,
    tags: Vec<String>,
    is_html: bool,
    db: State<Database>,
) -> Result<(), String> {
    db.update_snippet(&id, trigger, content, description, tags, is_html)
}

#[tauri::command]
pub fn delete_snippet(id: String, db: State<Database>) -> Result<(), String> {
    db.delete_snippet(&id)
}

#[tauri::command]
pub fn search_snippets(query: String, db: State<Database>) -> Result<Vec<Snippet>, String> {
    db.search_snippets(&query)
}

#[tauri::command]
pub fn export_snippets(db: State<Database>) -> Result<String, String> {
    db.export_snippets()
}

#[tauri::command]
pub fn import_snippets(json: String, db: State<Database>) -> Result<usize, String> {
    db.import_snippets(&json)
}

#[tauri::command]
pub fn increment_usage(id: String, db: State<Database>) -> Result<(), String> {
    db.increment_usage(&id)
}

#[tauri::command]
pub fn update_snippet_settings(
    time_delay_ms: u64,
    db: State<Database>,
) -> Result<(), String> {
    db.update_snippet_settings(time_delay_ms)
}

#[tauri::command]
pub fn get_snippets_settings(db: State<Database>) -> Result<SnippetSettings, String> {
    db.get_snippet_settings()
}