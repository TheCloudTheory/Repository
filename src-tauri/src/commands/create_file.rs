#[tauri::command]
pub fn create_file(file_name: &str) -> Result<(), String> {
    // We need to check if the file already exists
    if std::fs::metadata(&file_name).is_ok() {
        return Err("File already exists".to_string());
    }

    // We need to create the file
    std::fs::File::create(format!("{}.md", &file_name)).unwrap();

    Ok(())
}