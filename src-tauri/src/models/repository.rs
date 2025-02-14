use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Repository {
    pub general: General,
    pub files: HashMap<String, File>,
    pub directory: Option<Directory>
}

#[derive(Deserialize, Serialize)]
pub struct General {
    pub repository_name: String,
    pub created_on: String,
}

#[derive(Deserialize, Serialize)]
pub struct Directory {
    pub objects: Vec<DirectoryObject>,
}

#[derive(Deserialize, Serialize)]
pub struct DirectoryObject {
    pub is_directory: bool,
    pub name: String,
    pub objects: Vec<DirectoryObject>,
}

#[derive(Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub extension: String,
}