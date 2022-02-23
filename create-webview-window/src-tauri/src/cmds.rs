use std::{
    path::PathBuf,
    result::Result
};
use chrono::Utc;
use futures::future::join_all;
use tauri::{AppHandle, WindowBuilder};
use tauri::{
async_runtime::JoinHandle, Runtime, WindowUrl
};

#[tauri::command]
pub async fn create_multi_windows<R: Runtime>(app_handle: AppHandle<R>) -> Result<String, String> {
  let mut join_handles: Vec<JoinHandle<()>> = vec![];
  for i in 1..3 {
    let app_handle = app_handle.clone();
    join_handles.push(tauri::async_runtime::spawn(async move {
      println!("creating window-{:?}", i);
      do_create_window(app_handle);
    }));
  }
  join_all(join_handles).await;
  println!("all created.");
   Ok("{}".to_owned())
}

fn do_create_window<R: Runtime>(app_handle: AppHandle<R>) {
  app_handle
    .create_window(
    format!("window-{:?}", Utc::now().timestamp_millis()),
      WindowUrl::App(PathBuf::from("index.html")),
      |window_builder, webview_attributes| {
        let builder = window_builder
          .title("test".to_owned())
          .position(0.0, 0.0)
          .min_inner_size(600.0, 400.0)
          .inner_size(600.0, 400.0)
          .transparent(false)
          .decorations(true)
          .visible(false);
        (builder, webview_attributes)
      },
    )
    .unwrap();
}