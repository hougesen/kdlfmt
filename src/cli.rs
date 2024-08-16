use std::io::Read;

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
    /// Path to file OR directory.
    /// Use "-" to read from stdin.  
    #[arg()]
    pub input: Vec<String>,

    #[arg(long, value_enum)]
    pub log_level: Option<LogLevel>,
}

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

#[derive(Args, Debug)]
pub struct ShellCompletionCommandArguments {
    #[arg()]
    pub shell: clap_complete::Shell,
}

#[inline]
pub fn read_stdin() -> std::io::Result<String> {
    let stdin = std::io::stdin();

    let mut input = String::new();

    stdin.lock().read_to_string(&mut input)?;

    Ok(input)
}
