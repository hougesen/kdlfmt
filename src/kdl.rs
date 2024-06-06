#[inline]
pub fn parse_kdl(input: String) -> Result<kdl::KdlDocument, kdl::KdlError> {
    input.parse::<kdl::KdlDocument>()
}

#[inline]
pub fn format_kdl(mut input: kdl::KdlDocument) -> String {
    input.fmt();

    input.to_string()
}
