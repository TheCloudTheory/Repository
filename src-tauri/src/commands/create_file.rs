#[tauri::command]
pub fn create_file(file_name: &str) -> Result<(), String> {
    // We need to check if the file already exists
    if std::fs::metadata(&file_name).is_ok() {
        return Err("File already exists".to_string());
    }

    // The actual name of the file could be different than the name
    // of an entry displayed in the UI. By convention, we will replace
    // spaces with hyphens and make the name lowercase
    let file_name = file_name.replace(" ", "-").to_lowercase();

    // We need to create the file
    std::fs::File::create(format!("{}.md", &file_name)).unwrap();

    Ok(())
}