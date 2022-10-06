use crate::args::*;
use clap::Subcommand;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CLI {
    // Comando a ser executado
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    New(CommandNewQuery),
    Delete(CommandDeleteQuery),
    Toggle(CommandToggleQuery),
    Find(CommandFindQuery),
    List(CommandListQuery),
}
