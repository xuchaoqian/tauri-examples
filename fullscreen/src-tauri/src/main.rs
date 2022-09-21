#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::info;
use log4rs;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            _ => {
                info!("is_fullscreen: {:?}", event.window().is_fullscreen())
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
