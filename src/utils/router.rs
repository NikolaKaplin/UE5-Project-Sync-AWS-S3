use std::{fs};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use crate::utils::project::Project;

#[derive( Serialize, Deserialize, Debug)]
pub struct Router {
    pub current_project: Project,
}


impl Router {
    pub fn path_exist(path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    pub fn get_files_in_dir(dir_path: &str) -> Result<Vec<String>, std::io::Error> {
        let mut all_files = Vec::new();
        for entry in WalkDir::new(dir_path).max_depth(1) {
            let entry = entry?;
            all_files.push(entry.path().to_string_lossy().to_string());
        }
        Ok(all_files)
    }
}
