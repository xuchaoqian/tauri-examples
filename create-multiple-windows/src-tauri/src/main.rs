#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmds;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmds::create_multi_windows,
      cmds::show_multi_windows
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
