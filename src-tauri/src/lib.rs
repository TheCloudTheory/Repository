mod commands {
    pub mod list_files;
    pub mod create_file;
}

mod models {
    pub mod repository;
}

use commands::list_files::list_files;
use commands::create_file::create_file;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            list_files,
            create_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
