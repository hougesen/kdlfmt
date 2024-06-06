use clap::Parser;

use self::{cli::Cli, commands::execute_command, error::KdlFmtError};

mod cli;
mod commands;
mod error;
mod fs;
mod kdl;

fn main() -> Result<(), KdlFmtError> {
    let c = Cli::parse();

    if let Err(error) = execute_command(c.command) {
        eprintln!("{error:#?}");

        std::process::exit(1);
    }

    Ok(())
}
