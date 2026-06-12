//! Sig-Maker GUI — Tauri application
//!
//! In development, build with `--features dev-server` to start an HTTP server
//! at localhost:7331 for browser access and Playwright MCP automation.

// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

#[cfg(feature = "dev-server")]
mod http_server;

fn main() {
    #[cfg(feature = "dev-server")]
    std::thread::spawn(|| http_server::run());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_formats,
            commands::convert_pattern,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
