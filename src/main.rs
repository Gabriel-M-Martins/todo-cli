use std::{f32::consts::E, io::Error};

use clap::Parser;
use commands::Commands;
use task::Task;

mod args;
mod commands;
mod task;

pub const PATH_SAVE: &str = "C:\\Users\\Gabriel\\Documents\\todo\\";

fn main() {
    let args = args::Args::parse();

    match args.command {
        Commands::New(t) => {
            let tsk = Task::new(&t.name);
            match tsk.save(false) {
                Ok(_) => {
                    println!("'{}' foi salva.\n", tsk.name);
                    println!("{}", &tsk)
                }
                Err(e) => display_error(e),
            }
        }
        Commands::Edit(t) => {}
        Commands::Delete(t) => match Task::delete(&t.name) {
            Ok(_) => println!("Tarefa '{}' excluída com sucesso.", &t.name),
            Err(e) => display_error(e),
        },
        Commands::Toggle(t) => match Task::toggle(&t.name) {
            Ok(task) => {
                println!("Alternado o status da tarefa '{}'...", &task.name);

                match task.save(true) {
                    Ok(_) => println!("Tarefa salva com sucesso:\n\n{}", &task),
                    Err(e) => display_error(e),
                }
            }
            Err(e) => display_error(e),
        },
        Commands::Find(t) => match Task::find(&t.name) {
            Some(task) => println!("{}", task),
            None => println!("Task '{}' not found.", &t.name),
        },
        Commands::List => {
            let tasks_opt = Task::list();
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
    }
}

fn display_error(e: Error) {
    println!("{}", e.to_string())
}
