use std::{io};
use anyhow::{ Result};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::utils::input::MyHelper;
use crate::utils::project::Project;
use crate::utils::router::Router;
use crate::utils::unreal_engine::Engine;

mod tools;
mod functions;
mod utils;
#[tokio::main]
async fn main() -> Result<()> {
    utils::prints::print_logo();
    let mut current_project: String = Default::default();
    let mut item = format!("{}>>", &current_project);
    let projects = Engine::get_engine_projects()?;
    let project_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
    let helper = MyHelper::new(project_names);
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));
    loop {
        let readline = rl.readline(&item);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                match parts.as_slice() {
                    ["exit" | "quit"] => break,
                    ["path"] => {
                        get_all_files_in_dir().await?;
                    },
                    ["set", name] => {
                        current_project = name.to_string();
                        item = format!("{}>>", &current_project);
                        println!("Вы ввели: {}", line)
                    },

                    _ => println!("Вы ввели: {}", line),
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


