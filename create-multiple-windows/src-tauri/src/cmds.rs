use chrono::Utc;
use futures::future::join_all;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::{path::PathBuf, result::Result};
use tauri::{async_runtime::JoinHandle, Manager, Runtime, WindowUrl};
use tauri::{AppHandle, WindowBuilder};

static WINDOW_LABELS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(Vec::new()));

#[tauri::command]
pub async fn create_multi_windows<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
  let mut join_handles: Vec<JoinHandle<()>> = vec![];
  for n in 1..3 {
    let app_handle = app_handle.clone();
    join_handles.push(tauri::async_runtime::spawn(async move {
      let label = format!("window-{:?}", Utc::now().timestamp_millis());
      println!("creating window: {:?}", label);
      do_create_window(app_handle, label.clone(), n);
      WINDOW_LABELS.write().unwrap().push(label.clone());
      println!("created window: {:?}", label);
    }));
  }
  join_all(join_handles).await;
  println!("all created.");
  Ok(())
}

#[tauri::command]
pub async fn show_multi_windows<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
  {
    for label in &*WINDOW_LABELS.read().unwrap() {
      println!("showing window: {:?}", label);
      let window = app_handle.get_window(label).unwrap();
      window
        .show()
        .unwrap_or_else(|err| println!("Failed to show window: error: {:?}", err));
      window
        .set_focus()
        .unwrap_or_else(|err| println!("Failed to focus window: error: {:?}", err));
      println!("showed window: {:?}", label);
    }
  }
  WINDOW_LABELS.write().unwrap().clear();
  Ok(())
}

fn do_create_window<R: Runtime>(app_handle: AppHandle<R>, label: String, n: u32) {
  app_handle
    .create_window(
      label,
      WindowUrl::App(PathBuf::from("index.html")),
      |window_builder, webview_attributes| {
        let builder = window_builder
          .title("test".to_owned())
          .position((n as f64) * 20.0, (n as f64) * 20.0)
          .min_inner_size(600.0, 400.0)
          .inner_size(600.0, 400.0)
          .decorations(true)
          .visible(false);
        (builder, webview_attributes)
      },
    )
    .unwrap();
}
