use std::{fs, io::Write};
use chrono::prelude::*;
use crate::models::repository::{General, Repository};

#[tauri::command]
pub fn list_files() -> Result<Repository, String> {
    // In order to display the list of files we need to read the current directory
    // and see if there is a .repository directory and __repository.toml file. 
    // This file acts as the metadata store, so we can save some state regarding the repository. 
    // Otherwise, we create a new repository by creating the __repository.toml file.
    if std::fs::metadata(".repository").is_ok() == false {
        // Create the .repository directory
        fs::create_dir(".repository").unwrap();
    }

    if std::fs::metadata(".repository").is_ok() && std::fs::metadata(".repository/__repository.toml").is_ok() {
        // The repository metadata file exists, so we can read the content
        // and use it to provide some metadata plus the list of files in the
        // working directory (as it will be treated as a repository)
        let content = std::fs::read_to_string(".repository/__repository.toml").unwrap();
        let parsed: Repository = toml::from_str(&content).unwrap();
        
        return Ok(parsed);
    } else {
        // Create a new repository metadata file
        let mut file = std::fs::File::create(".repository/__repository.toml").unwrap();

        // Get the current date so we can annotate the repository creation date
        let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Write the initial content to the file which is basically
        // a dummy name of the repository
        let repository = Repository {
            general: General {
                repository_name: "New Repository".to_string(),
                created_on: date,
            },
            objects: Default::default(),
        };

        // Write the content to the file as a TOML string
        let content = toml::to_string(&repository).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        return Ok(repository);
    }
}
