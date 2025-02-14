use std::io::Write;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

// Define an array of supported file extensions
const SUPPORTED_EXTENSIONS: [&str; 1] = ["md"];

#[derive(Deserialize, Serialize)]
pub struct Repository {
    general: General,
    directory: Option<Directory>
}

#[derive(Deserialize, Serialize)]
pub struct General {
    repository_name: String,
    created_on: String,
}

#[derive(Deserialize, Serialize)]
pub struct Directory {
    objects: Vec<DirectoryObject>,
}

#[derive(Deserialize, Serialize)]
pub struct DirectoryObject {
    is_directory: bool,
    name: String,
    objects: Vec<DirectoryObject>,
}

#[tauri::command]
pub fn list_files() -> Result<Repository, String> {
    // In order to display the list of files we need to read the current directory
    // and see if there is a __repository.toml file. This file acts as the metadata
    // store, so we can save some state regarding the repository. Otherwise, we create
    // a new repository by creating the __repository.toml file.
    if std::fs::metadata("__repository.toml").is_ok() {
        // The repository metadata file exists, so we can read the content
        // and use it to provide some metadata plus the list of files in the
        // working directory (as it will be treated as a repository)
        let content = std::fs::read_to_string("__repository.toml").unwrap();
        let mut parsed: Repository = toml::from_str(&content).unwrap();

        // The metadata file doesn't contain infomation about the directory
        // structure hence we need to populate it by ourselves
        let mut objects = vec![];
        for entry in std::fs::read_dir(".").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let is_directory = path.is_dir();
            let name = entry.file_name().into_string().unwrap();

            // We don't want to include the repository metadata file
            // and files which are not supported by Repository
            if name == "__repository.toml" || !SUPPORTED_EXTENSIONS.iter().any(|&ext| name.ends_with(ext)) {
                continue;
            }

            objects.push(DirectoryObject {
                is_directory,
                name,
                objects: vec![],
            });
        }

        parsed.directory = Some(Directory {
            objects,
        });
        
        return Ok(parsed);
    } else {
        // Create a new repository metadata file
        let mut file = std::fs::File::create("__repository.toml").unwrap();

        // Get the current date so we can annotate the repository creation date
        let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Write the initial content to the file which is basically
        // a dummy name of the repository
        file.write_all(b"[general]\n").unwrap();
        file.write_all(b"repository_name = \"New Repository\"\n").unwrap();
        file.write_all(format!("created_on = \"{}\"\n", date).as_bytes()).unwrap();

        return Ok(Repository {
            general: General {
                repository_name: "New Repository".to_string(),
                created_on: date,
            },
            directory: Some(Directory {
                objects: vec![], // No files yet
            }),
        });
    }
}
