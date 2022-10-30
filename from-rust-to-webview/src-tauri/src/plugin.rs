use serde::Serialize;
use serde_json::to_string;
use std::{result::Result as StdResult, sync::Arc};
use tauri::{
    plugin::{Plugin, Result as PluginResult},
    AppHandle, Invoke, Manager, PageLoadPayload, Runtime, State, Window,
};

#[derive(Serialize)]
pub struct Row {
    pub v0: u32,
    pub v1: u32,
    pub v2: f64,
    pub v3: f64,
    pub v4: f64,
    pub v5: f64,
    pub v6: f64,
}

fn build_rows() -> Vec<Row> {
    let mut rows: Vec<Row> = vec![];
    for i in 0..200000 {
        rows.push(Row {
            v0: i,
            v1: i + 1,
            v2: (i + 2) as f64,
            v3: (i + 3) as f64,
            v4: (i + 4) as f64,
            v5: (i + 5) as f64,
            v6: (i + 6) as f64,
        })
    }
    rows
}

#[tauri::command]
pub async fn eval_data<R: Runtime>(
    _app_handle: AppHandle<R>,
    from_window: Window<R>,
    rows_as_json: State<'_, Arc<String>>,
    invoke_id: u32,
) -> Result<(), String> {
    from_window
        .eval(&format!(
            r#"
          if (!window.results) {{
            window.results = []
          }}
          window.results[{invoke_id}] = {result};
          "#,
            invoke_id = invoke_id,
            result = *rows_as_json,
        ))
        .unwrap();
    Ok(())
}

pub struct XPlugin<R: Runtime> {
    rows: Arc<Vec<Row>>,
    rows_as_json: Arc<String>,
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> XPlugin<R> {
    pub fn new() -> StdResult<Self, ()> {
        let rows = build_rows();
        let rows_as_json = to_string(&rows).unwrap();
        Ok(XPlugin {
            rows: Arc::new(rows),
            rows_as_json: Arc::new(rows_as_json),
            invoke_handler: Box::new(tauri::generate_handler![eval_data]),
        })
    }
}

impl<R: Runtime> Plugin<R> for XPlugin<R> {
    /// The plugin name. Must be defined and used on the `invoke` calls.
    fn name(&self) -> &'static str {
        "xplugin"
    }

    /// The JS script to evaluate on initialization.
    /// Useful when your plugin is accessible through `window`
    /// or needs to perform a JS task on app initialization
    /// e.g. "window.awesomePlugin = { ... the plugin interface }"
    fn initialization_script(&self) -> Option<String> {
        None
    }

    /// initialize plugin with the config provided on `tauri.conf.json > plugins > $yourPluginName` or the default value.
    fn initialize(&mut self, app: &AppHandle<R>, _: serde_json::Value) -> PluginResult<()> {
        app.manage(self.rows.clone());
        app.manage(self.rows_as_json.clone());
        Ok(())
    }

    /// Callback invoked when the Window is created.
    fn created(&mut self, _: Window<R>) {}

    /// Callback invoked when the webview performs a navigation.
    fn on_page_load(&mut self, _: Window<R>, _: PageLoadPayload) {}

    /// Extend the invoke handler.
    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}
