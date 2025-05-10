#[cfg(test)]
mod test_cli {
    use predicates::prelude::PredicateBooleanExt;
    use tempfile::tempdir;

    fn kdlfmt_command(path: &std::path::Path) -> assert_cmd::Command {
        let mut cmd = assert_cmd::Command::cargo_bin("kdlfmt").unwrap();

        cmd.current_dir(path);

        cmd
    }

    #[test]
    fn help_arg_outputs_message() {
        let dir = tempdir().unwrap();

        kdlfmt_command(dir.path())
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn version_arg_outputs_message() {
        let dir = tempdir().unwrap();

        kdlfmt_command(dir.path())
            .arg("--version")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    mod help {
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use crate::test_cli::kdlfmt_command;

        #[test]
        fn command_outputs_help_message() {
            let dir = tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }
    }

    mod completions {
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use crate::test_cli::kdlfmt_command;

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("completions")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }

        #[test]
        fn outputs_shell_completions() {
            let dir = tempdir().unwrap();

            let shells = ["bash", "elvish", "fish", "nushell", "powershell", "zsh"];

            for shell in shells {
                kdlfmt_command(dir.path())
                    .arg("completions")
                    .arg(shell)
                    .assert()
                    .success()
                    .stdout(predicates::str::is_empty().not());
            }
        }
    }

    mod init {
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use super::kdlfmt_command;

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("init")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }

        #[test]
        fn creates_a_config_file() {
            let dir = tempdir().unwrap();

            kdlfmt_command(dir.path()).arg("init").assert().success();

            let config_file_created = dir.path().join("kdlfmt.kdl").try_exists().unwrap();

            assert!(config_file_created);
        }

        #[test]
        fn fails_if_config_exists() {
            let dir = tempdir().unwrap();

            kdlfmt_command(dir.path()).arg("init").assert().success();

            kdlfmt_command(dir.path()).arg("init").assert().failure();
        }

        #[test]
        fn force_config_arg() {
            let dir = tempdir().unwrap();

            kdlfmt_command(dir.path()).arg("init").assert().success();

            kdlfmt_command(dir.path()).arg("init").assert().failure();

            kdlfmt_command(dir.path())
                .arg("init")
                .arg("--force")
                .assert()
                .success();
        }
    }
}
