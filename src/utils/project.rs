use std::{env, fs};
use std::error::Error;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub last_open_time: String
}

impl Project {
    pub fn validate_project(paths: &Vec<String>) -> bool {
        paths.iter().any(|path| {
            let path_obj = Path::new(path);
            path_obj.is_file() &&
                path_obj.extension().map_or(false, |ext| ext == "uproject")
        })
    }
    pub fn get_project_name(paths: &Vec<String>) -> String {
        paths.iter().find_map(|path| {
            let path_obj = Path::new(path);
            if path_obj.is_file() && path_obj.extension()? == "uproject" {
                path_obj.file_stem()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        }).expect("Не найден .uproject файл в переданных путях")
    }

    pub fn save_project_to_json(project_name: &str, path: &str) -> Result<(), Box<dyn Error>> {
        let json_path = Self::get_current_dir()?.join("projects.json");

        let mut projects = Self::load_projects(&json_path).unwrap_or_default();

        let new_project = Project {
            name: project_name.to_string(),
            path: path.to_string(),
            last_open_time: "none".to_string(),
        };

        if !projects.iter().any(|p| p.name == new_project.name && p.path == new_project.path) {
            projects.push(new_project);

            let json_data = serde_json::to_string_pretty(&projects)?;
            fs::write(json_path, json_data)?;
            println!("Проект успешно добавлен");
        } else {
            println!("Такой проект уже существует");
        }

        Ok(())
    }

    pub fn load_projects(json_path: &PathBuf) -> Result<Vec<Project>, Box<dyn Error>> {
        if json_path.exists() {
            let data = fs::read_to_string(json_path)?;
            serde_json::from_str(&data).map_err(|e| e.into())
        } else {
            Ok(Vec::new())
        }
    }

    pub fn get_current_dir() -> Result<PathBuf, Box<dyn Error>> {
        let exe_path = env::current_exe()?;
        Ok(exe_path.parent()
            .ok_or("Невозможно получить родительскую директорию")?
            .to_path_buf())
    }
}