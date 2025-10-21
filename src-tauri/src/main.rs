// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod config;
mod snippet;
mod snippet_settings;
mod typing;

use commands::*;
use config::Database;

fn main() {
    // Initialize database
    let db = Database::new().expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            get_all_snippets,
            create_snippet,
            update_snippet,
            delete_snippet,
            get_snippet_by_id,
            search_snippets,
            export_snippets,
            import_snippets,
            increment_usage,
            update_snippet_settings,
            get_snippet_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}