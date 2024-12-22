use crate::{
    cli::{Commands, LogLevel},
    config::KdlFmtConfig,
    error::KdlFmtError,
    terminal::logging::setup_logger,
};

mod check;
mod completions;
mod format;
mod init;

#[inline]
pub fn execute_command(command: Commands) -> Result<(), KdlFmtError> {
    match command {
        Commands::Check(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Debug));

            check::run(&args, &KdlFmtConfig::load()?)
        }
        Commands::Format(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Debug));

            format::run(&args, &KdlFmtConfig::load()?)
        }
        Commands::Completions(args) => {
            completions::run(&args);

            Ok(())
        }
        Commands::Init(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Debug));

            init::run()?;

            Ok(())
        }
    }
}
