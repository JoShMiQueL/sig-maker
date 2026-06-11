//! Sig-Maker GUI — Tauri application entry point

// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_formats,
            commands::convert_pattern,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
