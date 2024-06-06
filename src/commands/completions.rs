use clap::CommandFactory;

use crate::cli::{Cli, ShellCompletionCommandArguments};

#[inline]
pub fn run(args: &ShellCompletionCommandArguments) {
    let mut cmd = Cli::command();

    let bin_name = cmd.get_name().to_string();

    clap_complete::generate(args.shell, &mut cmd, bin_name, &mut std::io::stdout());
}
