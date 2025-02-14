use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Repository {
    pub general: General,
    pub objects: HashMap<String, DirectoryObject>,
}

#[derive(Deserialize, Serialize)]
pub struct General {
    pub repository_name: String,
    pub created_on: String,
}

#[derive(Deserialize, Serialize)]
pub struct DirectoryObject {
    pub is_directory: bool,
    pub name: String,
    pub objects: Vec<DirectoryObject>,
    pub extension: String,
}