use crate::{
    cli::FormatCommandArguments,
    error::KdlFmtError,
    fs::{read_file, save_file, setup_walker, KDL_FILE_EXTENSION},
    kdl::{format_kdl, parse_kdl},
    terminal::{print_format_changed_file, print_format_finished, print_format_unchanged_file},
};

#[inline]
pub fn run(args: &FormatCommandArguments) -> Result<(), KdlFmtError> {
    let walker = setup_walker(&args.input);

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
                .map_err(|error| KdlFmtError::ParseError(file_path.to_path_buf(), error))?;

            let formatted = format_kdl(parsed);

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
