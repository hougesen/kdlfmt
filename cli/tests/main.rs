use std::io::Write as _;

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

const INVALID_V1_CODE: &str = r#""""""""#;

const INVALID_V2_CODE: &str = r#""""""""#;

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

mod test_base_command {
    use predicates::prelude::PredicateBooleanExt;

    use crate::kdlfmt_command;

    #[test]
    fn help_arg_outputs_message() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path())
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn version_arg_outputs_message() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path())
            .arg("--version")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }
}

#[cfg(test)]
mod test_help_command {
    use predicates::prelude::PredicateBooleanExt;

    use crate::kdlfmt_command;

    #[test]
    fn command_outputs_help_message() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path())
            .arg("help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }
}

#[cfg(test)]
mod test_completions_command {
    use predicates::prelude::PredicateBooleanExt;

    use crate::kdlfmt_command;

    #[test]
    fn help_arg_outputs_message() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path())
            .arg("completions")
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn outputs_shell_completions() {
        let dir = tempfile::tempdir().unwrap();

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

#[cfg(test)]
mod test_init_command {
    use predicates::prelude::PredicateBooleanExt;

    use crate::kdlfmt_command;

    #[test]
    fn help_arg_outputs_message() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path())
            .arg("init")
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn creates_a_config_file() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path()).arg("init").assert().success();

        let config_file_created = dir.path().join("kdlfmt.kdl").try_exists().unwrap();

        assert!(config_file_created);
    }

    #[test]
    fn fails_if_config_exists() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path()).arg("init").assert().success();

        kdlfmt_command(dir.path()).arg("init").assert().failure();
    }

    #[test]
    fn force_config_arg() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path()).arg("init").assert().success();

        kdlfmt_command(dir.path()).arg("init").assert().failure();

        kdlfmt_command(dir.path())
            .arg("init")
            .arg("--force")
            .assert()
            .success();
    }
}

#[cfg(test)]
mod test_format_command {
    use predicates::prelude::PredicateBooleanExt;

    use crate::kdlfmt_command;

    #[test]
    fn help_arg_outputs_message() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path())
            .arg("format")
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    mod auto {
        use crate::{
            BROKEN_V1_CODE, BROKEN_V2_CODE, FORMATTED_V1_CODE, FORMATTED_V2_CODE, INVALID_V1_CODE,
            INVALID_V2_CODE, kdlfmt_command, setup_test_input,
        };

        #[test]
        fn formats_broken_code() {
            let dir = tempfile::tempdir().unwrap();

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
        fn accepts_multiple_paths() {
            let dir = tempfile::tempdir().unwrap();

            {
                let file1 = setup_test_input(dir.path(), BROKEN_V1_CODE);
                let file2 = setup_test_input(dir.path(), BROKEN_V1_CODE);

                kdlfmt_command(dir.path())
                    .arg("format")
                    .arg(file1.path())
                    .arg(file2.path())
                    .assert()
                    .success();

                {
                    let output = std::fs::read_to_string(file1.path()).unwrap();

                    assert_eq!(output, FORMATTED_V1_CODE);
                };

                {
                    let output = std::fs::read_to_string(file2.path()).unwrap();

                    assert_eq!(output, FORMATTED_V1_CODE);
                };
            };

            {
                let file1 = setup_test_input(dir.path(), BROKEN_V2_CODE);
                let file2 = setup_test_input(dir.path(), BROKEN_V2_CODE);

                kdlfmt_command(dir.path())
                    .arg("format")
                    .arg(file1.path())
                    .arg(file2.path())
                    .assert()
                    .success();
                {
                    let output = std::fs::read_to_string(file1.path()).unwrap();

                    assert_eq!(output, FORMATTED_V2_CODE);
                };

                {
                    let output = std::fs::read_to_string(file2.path()).unwrap();

                    assert_eq!(output, FORMATTED_V2_CODE);
                };
            };
        }

        #[test]
        fn formats_broken_code_stdin() {
            let dir = tempfile::tempdir().unwrap();

            {
                kdlfmt_command(dir.path())
                    .arg("format")
                    .arg("--stdin")
                    .write_stdin(BROKEN_V1_CODE)
                    .assert()
                    .success()
                    .stdout(predicates::str::contains(FORMATTED_V1_CODE));
            };

            {
                kdlfmt_command(dir.path())
                    .arg("format")
                    .arg("--stdin")
                    .write_stdin(BROKEN_V2_CODE)
                    .assert()
                    .success()
                    .stdout(predicates::str::contains(FORMATTED_V2_CODE));
            }
        }

        #[test]
        fn do_nothing_without_input() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path()).arg("format").assert().success();
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid() {
            let dir = tempfile::tempdir().unwrap();

            {
                let file = setup_test_input(dir.path(), INVALID_V1_CODE);

                kdlfmt_command(dir.path())
                    .arg("format")
                    .arg(file.path())
                    .assert()
                    .failure()
                    .stderr(predicates::str::contains("Error parsing file "));

                let output = std::fs::read_to_string(file.path()).unwrap();

                assert_eq!(output, INVALID_V1_CODE);
            };

            {
                let file = setup_test_input(dir.path(), INVALID_V2_CODE);

                kdlfmt_command(dir.path())
                    .arg("format")
                    .arg(file.path())
                    .assert()
                    .failure()
                    .stderr(predicates::str::contains("Error parsing file "));

                let output = std::fs::read_to_string(file.path()).unwrap();

                assert_eq!(output, INVALID_V2_CODE);
            };
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--stdin")
                .write_stdin(INVALID_V1_CODE)
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing content"));

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--stdin")
                .write_stdin(INVALID_V2_CODE)
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing content"));
        }
    }

    mod v1 {
        use crate::{
            BROKEN_V1_CODE, FORMATTED_V1_CODE, INVALID_V1_CODE, kdlfmt_command, setup_test_input,
        };

        #[test]
        fn formats_broken_code() {
            let dir = tempfile::tempdir().unwrap();

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
        fn formats_broken_code_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v1")
                .arg("--stdin")
                .write_stdin(BROKEN_V1_CODE)
                .assert()
                .success()
                .stdout(predicates::str::contains(FORMATTED_V1_CODE));
        }

        #[test]
        fn do_nothing_without_input() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v1")
                .assert()
                .success();
        }

        #[test]
        fn accepts_multiple_paths() {
            let dir = tempfile::tempdir().unwrap();

            let file1 = setup_test_input(dir.path(), BROKEN_V1_CODE);
            let file2 = setup_test_input(dir.path(), BROKEN_V1_CODE);

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v1")
                .arg(file1.path())
                .arg(file2.path())
                .assert()
                .success();

            {
                let output = std::fs::read_to_string(file1.path()).unwrap();

                assert_eq!(output, FORMATTED_V1_CODE);
            };

            {
                let output = std::fs::read_to_string(file2.path()).unwrap();

                assert_eq!(output, FORMATTED_V1_CODE);
            };
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid() {
            let dir = tempfile::tempdir().unwrap();

            let file = setup_test_input(dir.path(), INVALID_V1_CODE);

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v1")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing file "));

            let output = std::fs::read_to_string(file.path()).unwrap();

            assert_eq!(output, INVALID_V1_CODE);
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v1")
                .arg("--stdin")
                .write_stdin(INVALID_V1_CODE)
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing content"));
        }
    }

    mod v2 {
        use crate::{
            BROKEN_V2_CODE, FORMATTED_V2_CODE, INVALID_V2_CODE, kdlfmt_command, setup_test_input,
        };

        #[test]
        fn formats_broken_code() {
            let dir = tempfile::tempdir().unwrap();

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

        #[test]
        fn formats_broken_code_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v2")
                .arg("--stdin")
                .write_stdin(BROKEN_V2_CODE)
                .assert()
                .success()
                .stdout(predicates::str::contains(FORMATTED_V2_CODE));
        }

        #[test]
        fn do_nothing_without_input() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v2")
                .assert()
                .success();
        }

        #[test]
        fn accepts_multiple_paths() {
            let dir = tempfile::tempdir().unwrap();

            let file1 = setup_test_input(dir.path(), BROKEN_V2_CODE);
            let file2 = setup_test_input(dir.path(), BROKEN_V2_CODE);

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v2")
                .arg(file1.path())
                .arg(file2.path())
                .assert()
                .success();

            {
                let output = std::fs::read_to_string(file1.path()).unwrap();

                assert_eq!(output, FORMATTED_V2_CODE);
            };

            {
                let output = std::fs::read_to_string(file2.path()).unwrap();

                assert_eq!(output, FORMATTED_V2_CODE);
            };
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid() {
            let dir = tempfile::tempdir().unwrap();

            let file = setup_test_input(dir.path(), INVALID_V2_CODE);

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v2")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing file "));

            let output = std::fs::read_to_string(file.path()).unwrap();

            assert_eq!(output, INVALID_V2_CODE);
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("format")
                .arg("--kdl-version")
                .arg("v2")
                .arg("--stdin")
                .write_stdin(INVALID_V2_CODE)
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing content"));
        }
    }
}

#[cfg(test)]
mod test_check_command {
    use predicates::prelude::PredicateBooleanExt;

    use crate::kdlfmt_command;

    #[test]
    fn help_arg_outputs_message() {
        let dir = tempfile::tempdir().unwrap();

        kdlfmt_command(dir.path())
            .arg("check")
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    mod auto {
        use crate::{
            BROKEN_V1_CODE, BROKEN_V2_CODE, FORMATTED_V1_CODE, FORMATTED_V2_CODE, INVALID_V1_CODE,
            INVALID_V2_CODE, kdlfmt_command, setup_test_input,
        };

        #[test]
        fn success_with_formatted_input() {
            let dir = tempfile::tempdir().unwrap();

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
        fn success_with_formatted_input_stdin() {
            let dir = tempfile::tempdir().unwrap();

            {
                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg("--stdin")
                    .write_stdin(FORMATTED_V1_CODE)
                    .assert()
                    .success();
            };

            {
                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg("--stdin")
                    .write_stdin(FORMATTED_V2_CODE)
                    .assert()
                    .success();
            };
        }

        #[test]
        fn fail_with_unformatted_input() {
            let dir = tempfile::tempdir().unwrap();

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
        fn fail_with_unformatted_input_stdin() {
            let dir = tempfile::tempdir().unwrap();

            {
                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg("--stdin")
                    .write_stdin(BROKEN_V1_CODE)
                    .assert()
                    .failure();
            };

            {
                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg("--stdin")
                    .write_stdin(BROKEN_V2_CODE)
                    .assert()
                    .failure();
            };
        }

        #[test]
        fn do_nothing_without_input() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path()).arg("check").assert().success();
        }

        #[test]
        fn accepts_multiple_paths() {
            let dir = tempfile::tempdir().unwrap();

            {
                let file1 = setup_test_input(dir.path(), FORMATTED_V1_CODE);
                let file2 = setup_test_input(dir.path(), FORMATTED_V1_CODE);

                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg(file1.path())
                    .arg(file2.path())
                    .assert()
                    .success();
            };

            {
                let file1 = setup_test_input(dir.path(), FORMATTED_V2_CODE);
                let file2 = setup_test_input(dir.path(), FORMATTED_V2_CODE);

                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg(file1.path())
                    .arg(file2.path())
                    .assert()
                    .success();
            };
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid() {
            let dir = tempfile::tempdir().unwrap();

            {
                let file = setup_test_input(dir.path(), INVALID_V1_CODE);

                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg(file.path())
                    .assert()
                    .failure()
                    .stderr(predicates::str::contains("Error parsing file "));

                let output = std::fs::read_to_string(file.path()).unwrap();

                assert_eq!(output, INVALID_V1_CODE);
            };

            {
                let file = setup_test_input(dir.path(), INVALID_V2_CODE);

                kdlfmt_command(dir.path())
                    .arg("check")
                    .arg(file.path())
                    .assert()
                    .failure()
                    .stderr(predicates::str::contains("Error parsing file "));

                let output = std::fs::read_to_string(file.path()).unwrap();

                assert_eq!(output, INVALID_V2_CODE);
            };
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--stdin")
                .write_stdin(INVALID_V1_CODE)
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing content"));

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--stdin")
                .write_stdin(INVALID_V2_CODE)
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing content"));
        }
    }

    mod v1 {
        use crate::{
            BROKEN_V1_CODE, FORMATTED_V1_CODE, INVALID_V1_CODE, kdlfmt_command, setup_test_input,
        };

        #[test]
        fn success_with_formatted_input() {
            let dir = tempfile::tempdir().unwrap();

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
        fn success_with_formatted_input_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v1")
                .arg("--stdin")
                .write_stdin(FORMATTED_V1_CODE)
                .assert()
                .success();
        }

        #[test]
        fn fail_with_unformatted_input() {
            let dir = tempfile::tempdir().unwrap();

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
        fn fail_with_unformatted_input_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v1")
                .arg("--stdin")
                .write_stdin(BROKEN_V1_CODE)
                .assert()
                .failure();
        }

        #[test]
        fn do_nothing_without_input() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v1")
                .assert()
                .success();
        }

        #[test]
        fn accepts_multiple_paths() {
            let dir = tempfile::tempdir().unwrap();

            let file1 = setup_test_input(dir.path(), FORMATTED_V1_CODE);
            let file2 = setup_test_input(dir.path(), FORMATTED_V1_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v1")
                .arg(file1.path())
                .arg(file2.path())
                .assert()
                .success();
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid() {
            let dir = tempfile::tempdir().unwrap();

            let file = setup_test_input(dir.path(), INVALID_V1_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v1")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing file "));

            let output = std::fs::read_to_string(file.path()).unwrap();

            assert_eq!(output, INVALID_V1_CODE);
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v1")
                .arg("--stdin")
                .write_stdin(INVALID_V1_CODE)
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing content"));
        }
    }

    mod v2 {
        use crate::{
            BROKEN_V2_CODE, FORMATTED_V2_CODE, INVALID_V2_CODE, kdlfmt_command, setup_test_input,
        };

        #[test]
        fn success_with_formatted_input() {
            let dir = tempfile::tempdir().unwrap();

            let file = setup_test_input(dir.path(), FORMATTED_V2_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .arg(file.path())
                .assert()
                .success();
        }

        #[test]
        fn success_with_formatted_input_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .arg("--stdin")
                .write_stdin(FORMATTED_V2_CODE)
                .assert()
                .success();
        }

        #[test]
        fn fail_with_unformatted_input() {
            let dir = tempfile::tempdir().unwrap();

            let file = setup_test_input(dir.path(), BROKEN_V2_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .arg(file.path())
                .assert()
                .failure();
        }

        #[test]
        fn fail_with_unformatted_input_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .arg("--stdin")
                .write_stdin(BROKEN_V2_CODE)
                .assert()
                .failure();
        }

        #[test]
        fn do_nothing_without_input() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .assert()
                .success();
        }

        #[test]
        fn accepts_multiple_paths() {
            let dir = tempfile::tempdir().unwrap();

            let file1 = setup_test_input(dir.path(), FORMATTED_V2_CODE);
            let file2 = setup_test_input(dir.path(), FORMATTED_V2_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .arg(file1.path())
                .arg(file2.path())
                .assert()
                .success();
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid() {
            let dir = tempfile::tempdir().unwrap();

            let file = setup_test_input(dir.path(), INVALID_V2_CODE);

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing file "));

            let output = std::fs::read_to_string(file.path()).unwrap();

            assert_eq!(output, INVALID_V2_CODE);
        }

        #[test]
        fn it_should_fail_if_kdl_is_invalid_stdin() {
            let dir = tempfile::tempdir().unwrap();

            kdlfmt_command(dir.path())
                .arg("check")
                .arg("--kdl-version")
                .arg("v2")
                .arg("--stdin")
                .write_stdin(INVALID_V2_CODE)
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error parsing content"));
        }
    }
}
