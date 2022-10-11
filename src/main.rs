use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

use clap::Parser;
use commands::Commands;
use task::Task;

mod commands;
mod task;

fn main() {
    let path_save_dir = load_env();

    let cli = commands::CLI::parse();

    match cli.command {
        // -------------------------------------------------------------------------------------
        Some(command) => match command {
            // -------------------------------------------------------------------------------------
            Commands::New {task_name, overwrite} => {
                let tsk = Task::new(&task_name);
                match tsk.save(overwrite, &path_save_dir) {
                    Ok(_) => {
                        println!("'{}' foi salva.\n", tsk.name);
                        println!("{}", &tsk)
                    }
                    Err(e) => display_error(e),
                }
            }
            // -------------------------------------------------------------------------------------
            Commands::Delete {task_name, all} => match Task::delete(&task_name, path_save_dir, all) {
                Ok(_) => if all {
                    println!("Todas as tarefas excluídas com sucesso.")
                } else {
                    println!("Tarefa '{}' excluída com sucesso.", &task_name)
                }
                ,
                Err(e) => display_error(e),
            },
            // -------------------------------------------------------------------------------------
            Commands::Toggle {task_name, all} => match Task::toggle(&task_name, &path_save_dir, all) {
                Ok(task) => {
                    if let Some(task) = task{
                        match task.save(true, &path_save_dir) {
                            Ok(_) => println!("Tarefa salva com sucesso:\n\n{}", &task),
                            Err(e) => display_error(e),
                        }
                    } else {
                        println!("Todas as tarefas tiveram seu status alternado!");
                        list_tasks(path_save_dir)
                    }
                }
                Err(e) => display_error(e),
            },
            // -------------------------------------------------------------------------------------
            Commands::Find {task_name} => match Task::find(&task_name, &path_save_dir) {
                Some(task) => println!("{}", task),
                None => println!("Task '{}' not found.", task_name),
            },
            // -------------------------------------------------------------------------------------
        },
        // -------------------------------------------------------------------------------------
        None => list_tasks(path_save_dir),
    }
}

fn display_error(e: Error) {
    println!("{}", e.to_string())
}

fn list_tasks(path_save: PathBuf) {
    let tasks_opt = Task::list(&path_save);
    match tasks_opt {
        Some(task_vec) => {
            for task in task_vec {
                println!("{}", task)
            }
        }
        None => {
            println!("No task found.")
        }
    }
}

fn default_path_save_dir() -> Result<PathBuf, std::io::Error> {
    if let Some(mut home_dir) = home::home_dir() {
        home_dir.push(r"todo");
        Ok(home_dir)
    } else {
        Err(Error::new(
            ErrorKind::PermissionDenied,
            "No home directory.",
        ))
    }
}

//todo: return a env_config struct
fn load_env() -> PathBuf {
    dotenv::dotenv().ok();
    match dotenv::dotenv() {
        Ok(_) => {
            // -------------------------------------------------------------------------------------
            // load path to saved tasks
            if let Ok(value) = dotenv::var("PATH_SAVED_TASKS") {
                PathBuf::from(value)
            } else {
                match default_path_save_dir() {
                    Ok(value) => value,
                    Err(e) => panic!("Couldn't find 'PATH_SAVED_TASKS' in .env. Tried default directory at home dir, but an error ocurred. Error: {:?}.", e),
                }
            }
            // -------------------------------------------------------------------------------------

            // -------------------------------------------------------------------------------------
            // todo: load overwrite bool
        },
        Err(error_dotenv) => {
            match default_path_save_dir() {
                Ok(value) => value,
                Err(error_home_dir) => panic!("Couldn't find and/or read .env due. Tried default directory at home dir, but an error ocurred.\n Dotenv Error:\n {:?}.\n\n Home directory error:\n {:?}.", error_dotenv, error_home_dir),
            }   
        }
    }
}