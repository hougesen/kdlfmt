use std::str::FromStr;

use crate::{
    cli::{read_stdin, FormatCommandArguments},
    config::KdlFmtConfig,
    error::KdlFmtError,
    fs::{read_file, save_file, setup_walker, KDL_FILE_EXTENSION},
    kdl::{format_kdl, parse_kdl},
    terminal::{print_format_changed_file, print_format_finished, print_format_unchanged_file},
};

#[inline]
fn run_from_stdin(
    _args: &FormatCommandArguments,
    config: &KdlFmtConfig,
) -> Result<(), KdlFmtError> {
    let input = read_stdin().map_err(KdlFmtError::ReadStdinError)?;

    let parsed = parse_kdl(&input).map_err(|error| KdlFmtError::ParseError(None, error))?;

    let actual_config =
        KdlFmtConfig::get_editorconfig_or_default(config, &std::path::PathBuf::from("dummy.kdl"));

    let format_config = actual_config.get_formatter_config();

    let formatted = format_kdl(parsed, &format_config);

    println!("{formatted}");

    Ok(())
}

#[inline]
fn run_from_args(args: &FormatCommandArguments, config: &KdlFmtConfig) -> Result<(), KdlFmtError> {
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

    let overall_start_time = std::time::Instant::now();

    let mut file_count = 0;

    for entry in walker {
        let file_start_time = std::time::Instant::now();

        let file_path = entry.path();

        if file_path.is_file()
            && file_path
                .extension()
                .is_some_and(|ft| ft == KDL_FILE_EXTENSION)
        {
            let input = read_file(file_path)?;

            let parsed = parse_kdl(&input)
                .map_err(|error| KdlFmtError::ParseError(Some(file_path.to_path_buf()), error))?;

            let actual_config = KdlFmtConfig::get_editorconfig_or_default(
                config,
                &std::path::PathBuf::from(entry.path()),
            );

            let format_config = actual_config.get_formatter_config();

            let formatted = format_kdl(parsed, &format_config);

            save_file(file_path, &formatted).map_err(KdlFmtError::from)?;

            let time_elapsed = file_start_time.elapsed();

            if formatted == input {
                print_format_unchanged_file(file_path, time_elapsed);
            } else {
                print_format_changed_file(file_path, time_elapsed);
            }

            file_count += 1;
        }
    }

    print_format_finished(file_count, overall_start_time.elapsed());

    Ok(())
}

#[inline]
pub fn run(args: &FormatCommandArguments, config: &KdlFmtConfig) -> Result<(), KdlFmtError> {
    if args.input.len() == 1 && args.input.first().is_some_and(|v| v == "-") {
        run_from_stdin(args, config)
    } else {
        run_from_args(args, config)
    }
}
