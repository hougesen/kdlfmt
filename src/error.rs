#[derive(Debug)]
pub enum KdlFmtError {
    IoError(std::io::Error),
    InvalidPathError(String),
    ParseError(Option<std::path::PathBuf>, miette::Error),
    ReadStdinError(std::io::Error),
    CheckModeChanges,
}

impl std::fmt::Display for KdlFmtError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(error) => error.fmt(f),
            Self::ReadStdinError(error) => write!(f, "Error reading input from stdin - {error}"),
            Self::ParseError(maybe_path, error) => {
                if let Some(path) = maybe_path {
                    write!(f, "Error parsing file '{}' - {error:?}", path.display())
                } else {
                    write!(f, "Error parsing content - {error:?}")
                }
            }
            Self::InvalidPathError(path) => write!(f, "'{path}' is not a valid path"),
            Self::CheckModeChanges => write!(f, "Found changes while running in check mode"),
        }
    }
}

impl std::error::Error for KdlFmtError {}

impl From<std::io::Error> for KdlFmtError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
