use clap::Parser;
use commands::Commands;

mod args;
mod commands;
mod task;

pub const PATH_SAVE: &str = "C:\\Users\\Gabriel\\Documents\\todo\\";

fn main() {
    let args = args::Args::parse();

    match args.command {
        Commands::New(t) => {}
        Commands::Edit(t) => {}
        Commands::Delete(t) => {}
        Commands::Toggle(t) => {}
        Commands::Find(t) => {}
        Commands::List => {}
    }
}
