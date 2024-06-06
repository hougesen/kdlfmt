use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Format(FormatCommandArguments),
}

#[derive(Args, Debug)]
pub struct FormatCommandArguments {
    #[arg()]
    pub input: std::path::PathBuf,
}
