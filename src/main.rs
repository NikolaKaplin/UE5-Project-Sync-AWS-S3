use std::{io, thread};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use anyhow::{ Result};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::utils::input::MyHelper;
use crate::utils::prints::Prints;
use crate::utils::project::Project;
use crate::utils::router::Router;
use crate::utils::unreal_engine::Engine;

mod tools;
mod functions;
mod utils;
#[tokio::main]
async fn main() -> Result<()> {
    Prints::print_logo();

    let binding = &Default::default();
    let mut current_project: Option<&Project> = Some(binding);

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = stop_flag.clone();

    // Запускаем мониторинг в фоновом потоке
    thread::spawn(move || {
        Engine::monitor_unreal_editor(stop_flag_clone);
    });

    let mut console_path = format!("{}-> ", &current_project.unwrap().name);

    let projects = Engine::get_engine_projects()?;

    let project_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
    
    let helper = MyHelper::new(project_names.clone());
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));
    
    loop {
        let readline = rl.readline(&console_path);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                match parts.as_slice() {
                    ["path"] => {
                        get_all_files_in_dir().await?;
                    },
                    ["init"] => {
                        if *current_project.unwrap() != Project::default() {
                            // checking project conf
                            println!("Init project, created config, checking aws storage, init your branch.....");
                        }
                    },
                    ["pull"] => {
                        if *current_project.unwrap() != Project::default() {
                            println!("Pulling project {}....", &current_project.unwrap().name);
                        } else {
                            println!("Select project to use pull command... ");
                        }
                    },
                    ["push"] => {
                        if *current_project.unwrap() != Project::default()   {
                            println!("Pushing project {}....", &current_project.unwrap().name);
                        } else {
                            println!("Select project to use push command... ");
                        }
                    },
                    ["set", name] => {
                        if *current_project.unwrap() != Project::default() {
                            println!("Current project {}, use 'unset' to disable project", &current_project.unwrap().name);
                        } else if project_names.contains(&name.to_string()) {
                            current_project = projects.iter().find(|p| p.name == *name);
                            console_path = format!("{}-> ", &current_project.unwrap().name);
                        } else {
                            println!("Project not found: {}", line);
                        }
                    },
                    ["unset"] => {
                        if *current_project.unwrap() != Project::default()   {
                            current_project = None;
                            console_path = "-> ".to_string();
                        } else {
                            println!("Select project to use unset command... ");
                        }
                    },
                    ["start"] => {
                        if *current_project.unwrap() != Project::default()   {
                            let engine = "C:\\Program Files\\Epic Games\\UE_5.5\\Engine\\Binaries\\Win64\\UnrealEditor.exe";
                            Command::new(engine).spawn().expect("TODO: panic message");
                        } else {
                            println!("Select project to use start command... ");
                        }
                    },
                    ["help" | "h" | "-h"] => {
                        Prints::print_help();
                    },
                    ["exit" | "quit"] => break,
                    _ => println!("Command not found: {}, use 'help' to get more commands info", line),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Для выхода введите 'exit' или 'quit'");
            }
            Err(ReadlineError::Eof) => {
                println!("До свидания!");
                break;
            }
            Err(err) => {
                println!("Ошибка: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}



async fn get_all_files_in_dir() -> Result<Vec<String>> {
    println!("Enter path:");
    let mut path = String::new();
    let files = Vec::<String>::new();
    io::stdin().read_line(&mut path)?;
    let path = path.trim_end().to_string();
    println!("path exist: {}", Router::path_exist(&path));
    if Router::path_exist(&path) {
        let files =  Router::get_files_in_dir(&path)?;
        let is_ue_project = Project::validate_project(&files);
        println!("is_ue_project: {}", &is_ue_project);
        if is_ue_project {
            let project_name = Project::get_project_name(&files);
            println!("project_name: {}", &project_name);
            Project::save_project_to_json(&project_name, &path).expect("TODO: panic message");
            println!("save success");
        }
    } else { println!("path not found: {}", path) }
    Ok(files)
}


