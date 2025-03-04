#[tauri::command]
pub fn create_directory(directory_name: &str) -> Result<(), String> {
    // Check if the directory can be created
    if !crate::metadata::can_create_object(directory_name, true) {
        return Err("Directory already exists".to_string());
    }

    // Create the directory in the metadata
    crate::metadata::create_object(directory_name, true).unwrap();

    Ok(())
}