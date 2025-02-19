use crate::models::repository::Repository;

#[tauri::command]
pub fn open_file(key: &str) -> Result<String, String> {
    // First, load the repository metadata file so we can check if the provided
    // file key exists in the metadata. If it doesn't, return an error
    let metadata = std::fs::read_to_string(".repository/__repository.toml").unwrap();
    let parsed: Repository = toml::from_str(&metadata).unwrap();

    if parsed.objects.contains_key(key) == false {
        return Err("File does not exist".to_string());
    }

    // Check if the file exists. If it doesn't, return an error
    if std::fs::metadata(format!(".repository/{}.md", &key)).is_ok() == false {
        return Err("File does not exist".to_string());
    }

    // Read the content of the file and return it
    let content = std::fs::read_to_string(format!(".repository/{}.md", &key)).unwrap();
    Ok(content)
}