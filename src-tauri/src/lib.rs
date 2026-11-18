mod core;

use crate::core::http::HttpClient;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn check_api_connection(http: tauri::State<'_, HttpClient>) -> Result<String, String> {
    let response = http
        .client()
        .get("https://api.modrinth.com/v2/tag/game_version")
        .send()
        .await
        .map_err(|error| format!("Network error: {error}"))?;

    if response.status().is_success() {
        Ok("Modrinth API is reachable".to_string())
    } else {
        Err(format!("Modrinth returned HTTP {}", response.status()))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let http_client = HttpClient::new()?;
            app.manage(http_client);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, check_api_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
