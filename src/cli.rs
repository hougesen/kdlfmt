use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Format kdl files
    Format(FormatCommandArguments),

    /// Validate files are formatted
    Check(FormatCommandArguments),

    /// Generate shell completions
    Completions(ShellCompletionCommandArguments),
}

#[derive(Args, Debug)]
pub struct FormatCommandArguments {
    /// Path to file OR directory
    #[arg()]
    pub input: std::path::PathBuf,
}

#[derive(Args, Debug)]
pub struct ShellCompletionCommandArguments {
    #[arg()]
    pub shell: clap_complete::Shell,
}
