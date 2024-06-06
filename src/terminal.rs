use crate::error::KdlFmtError;

pub fn log_error(error: &KdlFmtError) {
    eprintln!("{error}");
}

#[inline]
pub fn log_formatting_finished(path: &std::path::Path) {
    println!("Finished formatting '{}'", path.display());
}
