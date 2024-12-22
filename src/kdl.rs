#[inline]
pub fn parse_kdl(input: &str) -> miette::Result<kdl::KdlDocument> {
    let parsed = kdl::KdlDocument::parse_v1(input)?;

    Ok(parsed)
}

#[inline]
pub fn format_kdl(mut input: kdl::KdlDocument, format_config: &kdl::FormatConfig) -> String {
    input.autoformat_config(format_config);

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

        let formatted = format_kdl(doc, &kdl::FormatConfig::default());

        assert_eq!(input, formatted);
    }
}
