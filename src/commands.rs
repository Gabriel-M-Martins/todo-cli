use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    New(TaskQuery),
    Delete(TaskQuery),
    Toggle(TaskQuery),
    Find(TaskQuery),
}

#[derive(Debug, Args)]
pub struct TaskQuery {
    pub query: String,
}
