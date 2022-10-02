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
            match tsk.save() {
                Ok(_) => println!("{} foi salva.", tsk.name),
                Err(e) => println!("Não foi possível salvar a tarefa. Erro: {:?}", e),
            }
        }
        Commands::Edit(t) => {}
        Commands::Delete(t) => {}
        Commands::Toggle(t) => {}
        Commands::Find(t) => {}
        Commands::List => {}
    }
}
