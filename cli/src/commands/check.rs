use std::str::FromStr;

use crate::{
    cli::{FormatCommandArguments, read_stdin},
    config::KdlFmtConfig,
    error::KdlFmtError,
    fs::{KDL_FILE_EXTENSION, read_file, setup_walker},
    kdl::{format_kdl, parse_kdl},
    terminal::print_check_changed_file,
};

#[inline]
fn run_from_stdin(args: &FormatCommandArguments, config: &KdlFmtConfig) -> Result<(), KdlFmtError> {
    let input = read_stdin().map_err(KdlFmtError::ReadStdinError)?;

    let (parsed, version) = parse_kdl(&input, args.kdl_version)
        .map_err(|error| KdlFmtError::ParseError(None, error))?;

    let actual_config =
        KdlFmtConfig::get_editorconfig_or_default(config, &std::path::PathBuf::from("dummy.kdl"));

    let format_config = actual_config.get_formatter_config();

    let formatted = format_kdl(parsed, &format_config, version);

    if input == formatted {
        Ok(())
    } else {
        Err(KdlFmtError::CheckModeChanges)
    }
}

#[inline]
pub fn run_from_args(
    args: &FormatCommandArguments,
    config: &KdlFmtConfig,
) -> Result<(), KdlFmtError> {
    let mut paths = Vec::new();

    for path in &args.input {
        paths.push(
            std::path::PathBuf::from_str(path)
                .map_err(|_| KdlFmtError::InvalidPathError(path.clone()))?,
        );
    }

    if paths.is_empty() {
        return Ok(());
    }

    let walker = setup_walker(paths);

    let mut file_count = 0;

    for entry in walker {
        let file_path = entry.path();

        if file_path.is_file()
            && file_path
                .extension()
                .is_some_and(|ft| ft == KDL_FILE_EXTENSION)
        {
            let input = read_file(file_path)?;

            let (parsed, version) = parse_kdl(&input, args.kdl_version)
                .map_err(|error| KdlFmtError::ParseError(Some(file_path.to_path_buf()), error))?;

            let actual_config = KdlFmtConfig::get_editorconfig_or_default(
                config,
                &std::path::PathBuf::from(entry.path()),
            );

            let format_config = actual_config.get_formatter_config();

            let formatted = format_kdl(parsed, &format_config, version);

            if formatted != input {
                print_check_changed_file(file_path);

                file_count += 1;
            }
        }
    }

    if file_count == 0 {
        Ok(())
    } else {
        Err(KdlFmtError::CheckModeChanges)
    }
}

#[inline]
pub fn run(args: &FormatCommandArguments, config: &KdlFmtConfig) -> Result<(), KdlFmtError> {
    if args.input.len() == 1 && args.input.first().is_some_and(|v| v == "-") {
        run_from_stdin(args, config)
    } else {
        run_from_args(args, config)
    }
}
