use crate::{cli::Commands, error::KdlFmtError};

mod check;
mod completions;
mod format;

#[inline]
pub fn execute_command(command: Commands) -> Result<(), KdlFmtError> {
    match command {
        Commands::Check(args) => check::run(&args),
        Commands::Format(args) => format::run(&args),
        Commands::Completions(args) => {
            completions::run(&args);
            Ok(())
        }
    }
}
