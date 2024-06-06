use crate::{
    cli::FormatCommandArguments,
    error::KdlFmtError,
    fs::{read_file, save_file, setup_walker, KDL_FILE_EXTENSION},
    kdl::{format_kdl, parse_kdl},
    terminal::{print_check_changed_file, print_check_finished},
};

#[inline]
pub fn run(args: &FormatCommandArguments) -> Result<(), KdlFmtError> {
    let walker = setup_walker(&args.input);

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
                .map_err(|error| KdlFmtError::ParseError(file_path.to_path_buf(), error))?;

            let formatted = format_kdl(parsed);

            save_file(file_path, &formatted).map_err(KdlFmtError::from)?;

            if formatted != input {
                print_check_changed_file(file_path);

                file_count += 1;
            }
        }
    }

    print_check_finished(file_count);

    Ok(())
}
