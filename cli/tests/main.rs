#[cfg(test)]
mod test_cli {
    use std::io::Write;

    use predicates::prelude::PredicateBooleanExt;
    use tempfile::tempdir;

    const BROKEN_V1_CODE: &str = r#"world          {
      child  "1"
 child                "2"
   }
"#;

    const FORMATTED_V1_CODE: &str = r#"world {
    child "1"
    child "2"
}
"#;

    const BROKEN_V2_CODE: &str = r#"world          {
      child  "1"
 child                "2"
   }
"#;

    const FORMATTED_V2_CODE: &str = r#"world {
    child "1"
    child "2"
}
"#;

    fn kdlfmt_command(path: &std::path::Path) -> assert_cmd::Command {
        let mut cmd = assert_cmd::Command::cargo_bin("kdlfmt").unwrap();

        cmd.current_dir(path);

        cmd
    }

    fn setup_test_input(dir: &std::path::Path, code: &str) -> tempfile::NamedTempFile {
        let mut b = tempfile::Builder::new();

        b.rand_bytes(12).suffix(".kdl");

        let mut f = b.tempfile_in(dir).unwrap();

        f.write_all(code.as_bytes()).unwrap();
        f.flush().unwrap();

        f
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

    mod format {
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use super::kdlfmt_command;
        use crate::test_cli::{
            BROKEN_V1_CODE, BROKEN_V2_CODE, FORMATTED_V1_CODE, FORMATTED_V2_CODE, setup_test_input,
        };

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }

        #[test]
        fn formats_broken_code() {
            let dir = tempdir().unwrap();

            {
                let file = setup_test_input(dir.path(), BROKEN_V1_CODE);

                kdlfmt_command(dir.path())
                    .arg("format")
                    .arg(file.path())
                    .assert()
                    .success();

                let output = std::fs::read_to_string(file.path()).unwrap();

                assert_eq!(output, FORMATTED_V1_CODE);
            };

            {
                let file = setup_test_input(dir.path(), BROKEN_V2_CODE);

                kdlfmt_command(dir.path())
                    .arg("format")
                    .arg(file.path())
                    .assert()
                    .success();

                let output = std::fs::read_to_string(file.path()).unwrap();

                assert_eq!(output, FORMATTED_V2_CODE);
            };
        }

        #[test]
        fn formats_broken_code_kdl_version_v1() {
            let dir = tempdir().unwrap();

            let file = setup_test_input(dir.path(), BROKEN_V1_CODE);

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v1")
                .arg(file.path())
                .assert()
                .success();

            let output = std::fs::read_to_string(file.path()).unwrap();

            assert_eq!(output, FORMATTED_V1_CODE);
        }

        #[test]
        fn formats_broken_code_kdl_version_v2() {
            let dir = tempdir().unwrap();

            let file = setup_test_input(dir.path(), BROKEN_V2_CODE);

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v2")
                .arg(file.path())
                .assert()
                .success();

            let output = std::fs::read_to_string(file.path()).unwrap();

            assert_eq!(output, FORMATTED_V2_CODE);
        }
    }

    mod check {
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use super::{
            BROKEN_V1_CODE, BROKEN_V2_CODE, FORMATTED_V1_CODE, FORMATTED_V2_CODE, kdlfmt_command,
            setup_test_input,
        };

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }

        #[test]
        fn success_with_formatted_input() {
            let dir = tempdir().unwrap();

            {
                let file = setup_test_input(dir.path(), FORMATTED_V1_CODE);

                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg(file.path())
                    .assert()
                    .success();
            };

            {
                let file = setup_test_input(dir.path(), FORMATTED_V2_CODE);

                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg(file.path())
                    .assert()
                    .success();
            };
        }

        #[test]
        fn success_with_formatted_input_kdl_version_v1() {
            let dir = tempdir().unwrap();

            let file = setup_test_input(dir.path(), FORMATTED_V1_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v1")
                .arg(file.path())
                .assert()
                .success();
        }

        #[test]
        fn success_with_formatted_input_kdl_version_v2() {
            let dir = tempdir().unwrap();

            let file = setup_test_input(dir.path(), FORMATTED_V2_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .arg(file.path())
                .assert()
                .success();
        }

        ///////
        ///
        ///
        ///

        #[test]
        fn fail_with_unformatted_input() {
            let dir = tempdir().unwrap();

            {
                let file = setup_test_input(dir.path(), BROKEN_V1_CODE);

                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg(file.path())
                    .assert()
                    .failure();
            };

            {
                let file = setup_test_input(dir.path(), BROKEN_V2_CODE);

                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg(file.path())
                    .assert()
                    .failure();
            };
        }

        #[test]
        fn fail_with_unformatted_kdl_version_v1() {
            let dir = tempdir().unwrap();

            let file = setup_test_input(dir.path(), BROKEN_V1_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v1")
                .arg(file.path())
                .assert()
                .failure();
        }

        #[test]
        fn fail_with_unformatted_kdl_version_v2() {
            let dir = tempdir().unwrap();

            let file = setup_test_input(dir.path(), BROKEN_V2_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .arg(file.path())
                .assert()
                .failure();
        }
    }
}
