use clap::Args;

#[derive(Debug, Args)]
pub struct CommandFindQuery {
    pub task_name: String,
}

#[derive(Debug, Args)]
pub struct CommandNewQuery {
    pub task_name: String,

    #[arg(short, long)]
    pub overwrite: bool,
}

#[derive(Debug, Args)]
pub struct CommandDeleteQuery {
    pub task_name: String,

    #[arg(short, long)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct CommandToggleQuery {
    pub task_name: String,

    #[arg(short, long)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct CommandListQuery {
    #[arg(short, long)]
    pub todo: bool,
    #[arg(short, long)]
    pub done: bool,
}
