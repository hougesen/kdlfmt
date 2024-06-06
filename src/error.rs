#[derive(Debug)]
pub enum KdlFmtError {
    IoError(std::io::Error),
    ParseError(std::path::PathBuf, kdl::KdlError),
}

impl std::fmt::Display for KdlFmtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => e.fmt(f),
            Self::ParseError(path, _error) => write!(f, "Error parsing file '{}'", path.display()),
        }
    }
}

impl std::error::Error for KdlFmtError {}

impl From<std::io::Error> for KdlFmtError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
