mod todo;
use clap::Parser;
use cli::Cli;

use crate::cli::Commands;

use anyhow::Result;

mod cli;
mod commands;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => {
            commands::add::run(task)?;
        }
        Commands::List => {
            commands::list::run()?;
        }
        Commands::Remove => {
            commands::remove::run()?;
        }
        Commands::Complete => {
            commands::complete::run()?;
        }
    }

    Ok(())
}
