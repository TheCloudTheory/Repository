mod commands {
    pub mod list_files;
    pub mod create_file;
    pub mod open_file;
    pub mod update_file_title;
}

mod models {
    pub mod repository;
}

use commands::list_files::list_files;
use commands::create_file::create_file;
use commands::open_file::open_file;
use commands::update_file_title::update_file_title;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_files,
            create_file,
            open_file,
            update_file_title
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
