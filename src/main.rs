use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

use clap::Parser;
use commands::Commands;
use task::Task;

mod args;
mod commands;
mod task;

fn main() {
    let path_save: PathBuf;
    dotenv::dotenv().ok();

    match dotenv::dotenv() {
        Ok(_) => {
            if let Ok(value) = dotenv::var("PATH_SAVED_TASKS") {
                path_save = PathBuf::from(value);
            } else {
                match default_path_save() {
                    Ok(value) => path_save = value,
                    Err(e) => panic!("Couldn't find 'PATH_SAVED_TASKS' in .env. Tried default directory at home dir, but an error ocurred. Error: {:?}.", e),
                }
            }
        },
        Err(error_dotenv) => {
            match default_path_save() {
                Ok(value) => path_save = value,
                Err(error_home_dir) => panic!("Couldn't find and/or read .env due. Tried default directory at home dir, but an error ocurred.\n Dotenv Error:\n {:?}.\n\n Home directory error:\n {:?}.", error_dotenv, error_home_dir),
            }   
        }
    }

    let args = args::Args::parse();

    match args.command {
        // -------------------------------------------------------------------------------------
        Some(command) => match command {
            // -------------------------------------------------------------------------------------
            Commands::New(t) => {
                let tsk = Task::new(&t.query);
                match tsk.save(false, path_save) {
                    Ok(_) => {
                        println!("'{}' foi salva.\n", tsk.name);
                        println!("{}", &tsk)
                    }
                    Err(e) => display_error(e),
                }
            }
            // -------------------------------------------------------------------------------------
            Commands::Delete(t) => match Task::delete(&t.query, path_save) {
                Ok(_) => println!("Tarefa '{}' excluída com sucesso.", &t.query),
                Err(e) => display_error(e),
            },
            // -------------------------------------------------------------------------------------
            Commands::Toggle(t) => match Task::toggle(&t.query, path_save.clone()) {
                Ok(task) => {
                    println!("Alternado o status da tarefa '{}'...", &task.name);

                    match task.save(true, path_save) {
                        Ok(_) => println!("Tarefa salva com sucesso:\n\n{}", &task),
                        Err(e) => display_error(e),
                    }
                }
                Err(e) => display_error(e),
            },
            // -------------------------------------------------------------------------------------
            Commands::Find(t) => match Task::find(&t.query, path_save) {
                Some(task) => println!("{}", task),
                None => println!("Task '{}' not found.", &t.query),
            },
            // -------------------------------------------------------------------------------------
            Commands::List => list_tasks(path_save),
        },
        // -------------------------------------------------------------------------------------
        None => list_tasks(path_save),
    }
}

fn display_error(e: Error) {
    println!("{}", e.to_string())
}

fn list_tasks(path_save: PathBuf) {
    let tasks_opt = Task::list(path_save);
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

fn default_path_save() -> Result<PathBuf, std::io::Error> {
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
