use std::f32::consts::E;

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
                    println!("{} foi salva.\n", tsk.name);
                    println!("{}", tsk)
                }
                Err(e) => println!("{}", e.to_string()),
            }
        }
        Commands::Edit(t) => {}
        Commands::Delete(t) => {}
        Commands::Toggle(t) => {}
        Commands::Find(t) => {}
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
