use assert_cmd::cargo::CargoError;
use predicates::prelude::PredicateBooleanExt;

fn completions_command() -> Result<assert_cmd::Command, CargoError> {
    let mut cmd = assert_cmd::Command::cargo_bin("kdlfmt")?;

    cmd.arg("completions");

    Ok(cmd)
}

#[test]
fn help_arg_outputs_message() -> Result<(), CargoError> {
    completions_command()?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::is_empty().not());

    Ok(())
}

#[test]
fn outputs_shell_completions() -> Result<(), CargoError> {
    let shells = ["bash", "elvish", "fish", "nushell", "powershell", "zsh"];

    for shell in shells {
        completions_command()?
            .arg(shell)
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    Ok(())
}
