use clap::Parser;

use self::{cli::Cli, commands::execute_command, terminal::print_error};

mod cli;
mod commands;
mod error;
mod fs;
mod kdl;
mod terminal;

fn main() {
    let cli = Cli::parse();

    if let Err(error) = execute_command(cli.command) {
        print_error(&error);

        std::process::exit(1);
    }
}
