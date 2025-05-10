use crate::{cli::Commands, config::KdlFmtConfig, error::KdlFmtError};

mod check;
mod completions;
mod format;
mod init;

#[inline]
pub fn execute_command(command: Commands) -> Result<(), KdlFmtError> {
    match command {
        Commands::Check(args) => check::run(&args, &KdlFmtConfig::load()?),
        Commands::Completions(args) => {
            completions::run(&args, &mut std::io::stdout());

            Ok(())
        }
        Commands::Format(args) => format::run(&args, &KdlFmtConfig::load()?),
        Commands::Init(args) => init::run(&args),
    }
}
