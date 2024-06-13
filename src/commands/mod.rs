use crate::{
    cli::{Commands, LogLevel},
    error::KdlFmtError,
    terminal::logging::setup_logger,
};

mod check;
mod completions;
mod format;

#[inline]
pub fn execute_command(command: Commands) -> Result<(), KdlFmtError> {
    match command {
        Commands::Check(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Debug));

            check::run(&args)
        }
        Commands::Format(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Debug));

            format::run(&args)
        }
        Commands::Completions(args) => {
            completions::run(&args);
            Ok(())
        }
    }
}
