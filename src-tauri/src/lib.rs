mod error;
mod http;
mod open_cloud;

pub use error::{Error, Result};

use open_cloud::{FullOpenCloudExecutionTask, OpenCloudExecutionTask, StructuredMessage};

#[tauri::command]
fn create_task_url(place_id: &str, universe_id: &str, version_number: Option<&str>) -> String {
    http::create_task_url(place_id, universe_id, version_number)
}

#[tauri::command]
async fn create_task(
    api_key: &str,
    task_url: &str,
    script: &str,
    timeout: Option<f64>,
) -> Result<OpenCloudExecutionTask> {
    http::create_task(api_key, task_url, script, timeout).await
}

#[tauri::command]
async fn await_task(api_key: &str, path: &str) -> Result<FullOpenCloudExecutionTask> {
    http::await_task(api_key, path).await
}

#[tauri::command]
async fn get_logs_flat(api_key: &str, path: &str) -> Result<Vec<String>> {
    http::get_logs_flat(api_key, path).await
}

#[tauri::command]
async fn get_logs_structured(api_key: &str, path: &str) -> Result<Vec<StructuredMessage>> {
    http::get_logs_structured(api_key, path).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_task,
            create_task_url,
            await_task,
            get_logs_flat,
            get_logs_structured
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
