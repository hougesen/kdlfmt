use crate::{
    cli::FormatCommandArguments,
    error::KdlFmtError,
    fs::{read_file, save_file},
    kdl::{format_kdl, parse_kdl},
};

#[inline]
pub fn run(args: FormatCommandArguments) -> Result<(), KdlFmtError> {
    let input = read_file(&args.input)?;

    let parsed = parse_kdl(input)?;

    let formatted = format_kdl(parsed);

    save_file(&args.input, &formatted).map_err(KdlFmtError::from)
}
