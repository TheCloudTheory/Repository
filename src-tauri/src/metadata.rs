use crate::models::repository::Repository;

pub fn get_metadata() -> Result<Repository, String> {
    let metadata = std::fs::read_to_string(".repository/__repository.toml");
    match metadata {
        Ok(metadata) => {
            let parsed: Repository = toml::from_str(&metadata).unwrap();
            Ok(parsed)
        },
        Err(_) => {
            Err("Metadata file not found".to_string())
        }
    }
}

pub fn save_metadata(metadata: &Repository) -> Result<(), String> {
    let serialized = toml::to_string(metadata).unwrap();
    let result = std::fs::write(".repository/__repository.toml", serialized);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Error saving metadata file".to_string())
    }
}

pub fn can_create_object(key: &str, is_directory: bool) -> bool {
    let metadata = get_metadata().unwrap();
    let object = metadata.objects.get(key);

    match object {
        Some(o) => {
            if o.is_directory && is_directory {
                // Check if the directory already exists. If it does, return an error.
                if std::path::Path::new(&format!(".repository/{}", key)).exists() {
                    false
                } else {
                    true
                }
            } else {
                // Check if the file already exists. If it does, return an error.
                if std::path::Path::new(&format!(".repository/{}.md", key)).exists() {
                    false
                } else {
                    true
                }
            }
        },
        None => true
    }
}

pub fn create_object(key: &str, is_directory: bool) -> Result<(), String> {
    let metadata = get_metadata().unwrap();
    let mut objects = metadata.objects;
    let object = crate::models::repository::DirectoryObject {
        is_directory,
        name: key.to_string(),
        objects: vec![],
        extension: "".to_string(),
    };

    objects.insert(key.to_string(), object);
    let new_metadata = Repository {
        general: metadata.general,
        objects,
    };

    save_metadata(&new_metadata)
}