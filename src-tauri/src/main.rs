// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod request;
mod config;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![request::request_weather_info])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application!");
}
