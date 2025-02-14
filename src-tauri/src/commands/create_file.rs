use std::io::Write;
use crate::models::repository::{Repository, DirectoryObject};

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

    // We also need to update the metadata file and put a reference
    // to the new file there. This will help in keeping the actual
    // file name detached from the UI and other information stored there
    let metadata = std::fs::read_to_string("__repository.toml").unwrap();
    let mut parsed: Repository = toml::from_str(&metadata).unwrap();
    parsed.objects.insert(file_name.clone(), DirectoryObject {
        name: file_name.clone(),
        extension: "md".to_string(),
        is_directory: false,
        objects: Default::default(),
    });

    // In the end we need to update the metadata file
    let new_metadata = toml::to_string(&parsed).unwrap();
    let mut file = std::fs::File::create("__repository.toml").unwrap();
    file.write_all(new_metadata.as_bytes()).unwrap();

    Ok(())
}