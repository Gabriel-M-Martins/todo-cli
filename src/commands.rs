use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    New(TaskQuery),
    Edit(TaskQuery),
    Delete(TaskQuery),
    Toggle(TaskQuery),
    Find(TaskQuery),
    List,
}

#[derive(Debug, Args)]
pub struct TaskQuery {
    pub name: String,
}
