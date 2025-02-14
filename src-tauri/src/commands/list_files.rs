use std::io::Write;
use chrono::prelude::*;
use crate::models::repository::{General, Repository, Directory, DirectoryObject};

// Define an array of supported file extensions
const SUPPORTED_EXTENSIONS: [&str; 1] = ["md"];

#[tauri::command]
pub fn list_files() -> Result<Repository, String> {
    // In order to display the list of files we need to read thśe current directory
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
            let mut name = entry.file_name().into_string().unwrap();

            // We don't want to include the repository metadata file
            // and files which are not supported by Repository
            if name == "__repository.toml" || !SUPPORTED_EXTENSIONS.iter().any(|&ext| name.ends_with(ext)) {
                continue;
            }

            // If the object is a file, we need to remove the extension
            // from the name so it looks better in the UI
            if !is_directory {
                name = name.split('.').collect::<Vec<&str>>()[0].to_string();
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
        file.write_all(b"\n[files]\n").unwrap();

        return Ok(Repository {
            general: General {
                repository_name: "New Repository".to_string(),
                created_on: date,
            },
            files: Default::default(),
            directory: Some(Directory {
                objects: vec![], // No files yet
            }),
        });
    }
}
