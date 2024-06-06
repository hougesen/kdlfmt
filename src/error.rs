#[derive(Debug)]
pub enum KdlFmtError {
    IoError(std::io::Error),
    ParseError(kdl::KdlError),
}

impl std::fmt::Display for KdlFmtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => e.fmt(f),
            Self::ParseError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for KdlFmtError {}

impl From<std::io::Error> for KdlFmtError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<kdl::KdlError> for KdlFmtError {
    fn from(value: kdl::KdlError) -> Self {
        Self::ParseError(value)
    }
}
