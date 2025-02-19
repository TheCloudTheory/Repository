use crate::models::repository::Repository;

#[tauri::command]
pub fn update_file_title(key: &str, file_title: &str) -> Result<String, String> {
    // First, load the repository metadata file so we can check if the provided
    // file key exists in the metadata. If it doesn't, return an error
    let metadata = std::fs::read_to_string(".repository/__repository.toml").unwrap();
    let mut parsed: Repository = toml::from_str(&metadata).unwrap();

    // Check if the file key is in the metadata file. If it isn't, return an error
    let file = parsed.objects.get_mut(key);
    if file.is_none() {
        return Err("File does not exist".to_string());
    }

    // Check if the file exists. If it doesn't, return an error
    if std::fs::metadata(format!(".repository/{}.md", &key)).is_ok() == false {
        return Err("File does not exist".to_string());
    }

    // Update the title of the file in the metadata file
    file.unwrap().name = file_title.to_string();

    // Save the updated metadata file
    let toml = toml::to_string(&parsed).unwrap();
    std::fs::write(".repository/__repository.toml", toml).unwrap();

    Ok("File title updated".to_string())
}