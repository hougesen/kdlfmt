use assert_cmd::cargo::CargoError;
use predicates::prelude::PredicateBooleanExt;

fn kdlfmt_command(path: Option<&std::path::Path>) -> Result<assert_cmd::Command, CargoError> {
    let mut cmd = assert_cmd::Command::cargo_bin("kdlfmt")?;

    if let Some(path) = path {
        cmd.current_dir(path);
    }

    Ok(cmd)
}

#[test]
fn base_help_arg_outputs_message() -> Result<(), CargoError> {
    kdlfmt_command(None)?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::is_empty().not());

    Ok(())
}

#[test]
fn base_version_arg_outputs_message() -> Result<(), CargoError> {
    kdlfmt_command(None)?
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::is_empty().not());

    Ok(())
}

#[test]
fn help_command_outputs_help_message() -> Result<(), CargoError> {
    kdlfmt_command(None)?
        .arg("help")
        .assert()
        .success()
        .stdout(predicates::str::is_empty().not());

    Ok(())
}
