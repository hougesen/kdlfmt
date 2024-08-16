use std::str::FromStr;

use crate::{
    cli::{read_stdin, FormatCommandArguments},
    error::KdlFmtError,
    fs::{read_file, setup_walker, KDL_FILE_EXTENSION},
    kdl::{format_kdl, parse_kdl},
    terminal::print_check_changed_file,
};

#[inline]
fn run_from_stdin(_args: &FormatCommandArguments) -> Result<(), KdlFmtError> {
    let input = read_stdin().map_err(KdlFmtError::ReadStdinError)?;

    let parsed = parse_kdl(&input).map_err(|error| KdlFmtError::ParseError(None, error))?;

    let formatted = format_kdl(parsed);

    if input == formatted {
        Ok(())
    } else {
        Err(KdlFmtError::CheckModeChanges)
    }
}

#[inline]
pub fn run_from_args(args: &FormatCommandArguments) -> Result<(), KdlFmtError> {
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

            let parsed = parse_kdl(&input)
                .map_err(|error| KdlFmtError::ParseError(Some(file_path.to_path_buf()), error))?;

            let formatted = format_kdl(parsed);

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
pub fn run(args: &FormatCommandArguments) -> Result<(), KdlFmtError> {
    if args.input.len() == 1 && args.input.first().is_some_and(|v| v == "-") {
        run_from_stdin(args)
    } else {
        run_from_args(args)
    }
}
