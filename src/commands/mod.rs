use crate::{
    cli::{Commands, LogLevel},
    config::KdlFmtConfig,
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

            let config = KdlFmtConfig::load()?;

            let indent = config.get_indent();
            let format_config = kdl::FormatConfig::builder().indent(&indent).build();

            check::run(&args, format_config)
        }
        Commands::Format(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Debug));

            let config = KdlFmtConfig::load()?;

            let indent = config.get_indent();
            let format_config = kdl::FormatConfig::builder().indent(&indent).build();

            format::run(&args, format_config)
        }
        Commands::Completions(args) => {
            completions::run(&args);

            Ok(())
        }

        Commands::Init(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Debug));

            KdlFmtConfig::init()?;

            Ok(())
        }
    }
}
