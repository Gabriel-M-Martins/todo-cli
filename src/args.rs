use clap::Parser;

use crate::commands::Commands;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    // Comando a ser executado
    #[clap(subcommand)]
    pub command: Option<Commands>,
}
