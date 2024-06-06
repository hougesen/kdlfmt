use std::str::FromStr;

use error::KdlFmtError;

mod error;

#[inline]
fn read_file(path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

#[inline]
fn parse_kdl(input: String) -> Result<kdl::KdlDocument, kdl::KdlError> {
    input.parse::<kdl::KdlDocument>()
}

#[inline]
fn format_kdl(mut input: kdl::KdlDocument) -> String {
    input.fmt();

    input.to_string()
}

fn main() -> Result<(), KdlFmtError> {
    let path = std::path::PathBuf::from_str("config.kdl").unwrap();

    let input = read_file(&path)?;

    let parsed = parse_kdl(input)?;

    let formatted = format_kdl(parsed);

    std::fs::write("config.formatted.kdl", formatted)?;

    Ok(())
}
