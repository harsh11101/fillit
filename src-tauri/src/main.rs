// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod config;
mod snippet;
mod snippet_settings;
mod keyboard_handler;
mod clipboard_handler;

use command::*;
use config::Database;
use keyboard_handler::KeyboardHandler;
use std::sync::Arc;

fn main() {
    let db = Arc::new(Database::new().expect("Failed to initialize database"));
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .manage(db.clone())
        .setup(move |_app| {
            let db_for_keyboard = db.clone();
            std::thread::spawn(move || {
                let keyboard_handler = Arc::new(KeyboardHandler::new(db_for_keyboard));
                keyboard_handler.start_listening();
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            });
            Ok(())
        })
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
            get_snippets_settings
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            match event {
                tauri::RunEvent::ExitRequested { api, .. } => {
                    api.prevent_exit();
                }
                _ => {}
            }
        });
}