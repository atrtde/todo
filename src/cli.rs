use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Simple Todo CLI", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg()]
        task: Option<String>,
    },
    List,
    Remove,
    Complete,
}
