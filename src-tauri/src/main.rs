// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    open_cloud_luau_execute_lib::config::load_config()
        .await
        .unwrap();
    open_cloud_luau_execute_lib::run()
}
