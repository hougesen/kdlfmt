use std::io::Read;

use clap::{Args, Parser, Subcommand};

const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{about-with-newline}{author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true, help_template = HELP_TEMPLATE)]
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

    /// Initialize formatter config
    Init(InitCommandArguments),

    /// Generate shell completions
    Completions(ShellCompletionCommandArguments),
}

#[derive(Args, Debug)]
pub struct FormatCommandArguments {
    /// Path to file OR directory.
    ///
    /// Use "-" to read from stdin and print to stdout.
    #[arg()]
    pub input: Vec<String>,

    /// kdl specification to use.
    ///
    /// By default all versions are tried
    #[arg(long, value_enum)]
    pub kdl_version: Option<KdlVersion>,

    /// Read from stdin and print to stdout.
    #[arg(long)]
    pub stdin: bool,

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

#[derive(Args, Debug)]
pub struct InitCommandArguments {
    /// Path to file OR directory.
    ///
    /// Use "-" to read from stdin and print to stdout.
    #[arg()]
    pub input: Vec<String>,

    /// kdl specification to use.
    ///
    /// By default all versions are tried
    #[arg(long, value_enum)]
    pub kdl_version: Option<KdlVersion>,

    /// Read from stdin and print to stdout.
    #[arg(long)]
    pub stdin: bool,

    #[arg(long, value_enum)]
    pub log_level: Option<LogLevel>,
}

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum KdlVersion {
    V1,
    #[default]
    V2,
}
