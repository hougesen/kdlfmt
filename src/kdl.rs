#[inline]
pub fn parse_kdl(input: &str) -> miette::Result<kdl::KdlDocument> {
    let parsed = input.parse::<kdl::KdlDocument>()?;

    Ok(parsed)
}

#[inline]
pub fn format_kdl(mut input: kdl::KdlDocument) -> String {
    input.fmt();

    input.to_string()
}

#[cfg(test)]
mod test {
    use super::parse_kdl;
    use crate::kdl::format_kdl;

    #[test]
    fn it_should_be_reversible() {
        let input = r#"world {
    child "1"
    child "2"
}
"#;

        let doc = parse_kdl(input).expect("it to parse valid kdl");

        let formatted = format_kdl(doc);

        assert_eq!(input, formatted);
    }
}
