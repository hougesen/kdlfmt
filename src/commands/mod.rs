use crate::{cli::Commands, error::KdlFmtError};

pub mod format;

#[inline]
pub fn execute_command(command: Commands) -> Result<(), KdlFmtError> {
    match command {
        Commands::Format(args) => format::run(&args),
    }
}
