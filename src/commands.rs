use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CLI {
    // Comando a ser executado
    #[clap(subcommand)]
    pub command: Option<Commands>,

    #[arg(short = 't')]
    pub only_todo: bool,
    #[arg(short = 'c')]
    pub only_completed: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    New {
        #[arg(short, long)]
        overwrite: bool,
        task_name: String,
    },

    #[command(arg_required_else_help = true)]
    Find { task_name: String },

    #[command(arg_required_else_help = true)]
    Delete {
        #[arg(short, long)]
        all: bool,
        #[arg(default_value_t = String::new())]
        task_name: String,
    },

    #[command(arg_required_else_help = true)]
    Toggle {
        #[arg(short, long)]
        all: bool,
        #[arg(default_value_t = String::new())]
        task_name: String,
    },
}
