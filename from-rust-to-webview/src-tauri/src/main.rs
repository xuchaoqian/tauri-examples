#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use plugin::XPlugin;

mod plugin;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(XPlugin::new().unwrap())
        .invoke_handler(tauri::generate_handler![greet, plugin::eval_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
