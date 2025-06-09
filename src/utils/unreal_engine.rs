use std::thread;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::{Project};
use crate::utils::router::Router;
use crate::utils::user::User;
use ini::Ini;
use notify_rust::Notification;
use sysinfo::System;

#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    pub versions: Vec<String>,
    pub projects: Vec<Project>
}

impl Engine {
    pub fn get_engine_versions() -> Result<Vec<String>, std::io::Error> {
        let path = "C:\\Program Files\\Epic Games";
        let mut versions = Vec::new();
        if Router::path_exist(path) {
            let files: Vec<_> = Router::get_files_in_dir(path)?.iter().map(|f| f.split("\\").last().unwrap().to_string()).collect();
            versions = files
                .iter()
                .filter(|e| e.to_lowercase().contains("ue_"))
                .map(|e| e.split('_').last().unwrap().to_string())
                .collect();
        }
        Ok(versions)
    }

    pub fn get_engine_projects() -> Result<Vec<Project>, std::io::Error> {
        let path = format!("C:\\Users\\{}\\AppData\\Local\\UnrealEngine\\5.5\\Saved\\Config\\WindowsEditor", User::get_user_name());
        let mut projects: Vec<Project> = Vec::new();

        if !Router::path_exist(&path) {
            return Ok(projects);
        }

        let conf = match Ini::load_from_file(format!("{}\\EditorSettings.ini", &path)) {
            Ok(c) => c,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
        };

        let section = Some("/Script/UnrealEd.EditorSettings");
        let key_project = "RecentlyOpenedProjectFiles";

        for (sec, prop) in &conf {
            if sec != section {
                continue;
            }

            for (key, value) in prop.iter() {
                if key != key_project {
                    continue;
                }

                let s = value.trim().trim_start_matches('(').trim_end_matches(')');
                let mut parts = s.split(',');

                let project_part = match parts.next() {
                    Some(p) => p,
                    None => continue,
                };

                let project_path = project_part
                    .trim()
                    .trim_start_matches("ProjectName=\"")
                    .trim_end_matches('\"')
                    .to_string();

                let name = match Path::new(&project_path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                {
                    Some(n) => n.to_string(),
                    None => continue,
                };

                let path = match Path::new(&project_path)
                    .parent()
                    .and_then(|p| p.to_str())
                {
                    Some(p) => p.to_string(),
                    None => continue,
                };

                let last_open_time = match parts.next() {
                    Some(t) => t.trim().trim_start_matches("LastOpenTime=").to_string(),
                    None => String::new(),
                };

                projects.push(Project {
                    name,
                    path,
                    last_open_time,
                });
            }
        }

        Ok(projects)
    }

    pub fn find_ue_process() -> String {
        let mut system = System::new_all();
        system.refresh_all();
        system.processes()
            .iter()
            .find(|(_, process)| process.name() == "UnrealEditor.exe")
            .map(|(&pid, _)| pid.to_string())
            .unwrap_or_else(|| "".to_string())
    }

    pub fn check_ue_process() -> String {
        let check_process = thread::spawn(move || {
            loop {
                let process= Self::find_ue_process();
                sleep(Duration::from_secs(10));
                return process;
            }
        });
        check_process.join().unwrap()
    }
    pub fn monitor_unreal_editor(stop_flag: Arc<AtomicBool>) {
        let mut system = System::new_all();
        let mut was_running = false;

        while !stop_flag.load(Ordering::Relaxed) {
            system.refresh_all();

            let is_running = system.processes_by_name("UnrealEditor.exe".as_ref()).next().is_some();

            match (is_running, was_running) {
                (true, false) => {
                    Notification::new()
                        .summary("RsGet AWS Sync")
                        .body("UnrealEditor.exe запущен!")
                        .show()
                        .unwrap_or_else(|e| eprintln!("Ошибка: {:?}", e));
                    was_running = true;
                },
                (false, true) => {
                    Notification::new()
                        .summary("RsGet AWS Sync")
                        .body("UnrealEditor.exe запущен!")
                        .show()
                        .unwrap_or_else(|e| eprintln!("Ошибка: {:?}", e));
                    was_running = false;
                },
                _ => {}
            }

            thread::sleep(Duration::from_secs(10));
        }
    }
}