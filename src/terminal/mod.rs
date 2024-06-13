use console::style;
use log::{error, info, warn};

use crate::error::KdlFmtError;

pub mod logging;

#[inline]
pub fn print_error(error: &KdlFmtError) {
    error!("{error}");
}

#[inline]
pub fn print_format_changed_file(path: &std::path::Path, duration: core::time::Duration) {
    info!("{} finished in {}ms", path.display(), duration.as_millis());
}

#[inline]
pub fn print_format_unchanged_file(path: &std::path::Path, duration: core::time::Duration) {
    info!(
        "{}",
        style(format!(
            "{} finished in {}ms (unchanged)",
            path.display(),
            duration.as_millis()
        ))
        .dim()
    );
}

#[inline]
pub fn print_format_finished(file_count: usize, duration: core::time::Duration) {
    let millis = duration.as_millis();

    let time_taken = if millis > 1000 {
        format!("{}s", duration.as_secs())
    } else {
        format!("{millis}ms")
    };

    let file_or_files = if file_count == 1 { "file" } else { "files" };

    info!("{file_count} {file_or_files} was formatted in {time_taken}");
}

#[inline]
pub fn print_check_changed_file(path: &std::path::Path) {
    warn!(
        "{}",
        style(format!("{} is not formatted", path.display()))
            .bold()
            .red()
    );
}

#[inline]
pub fn print_check_finished(file_count: usize) {
    if file_count == 0 {
        info!("All files are formatted!");
    } else {
        let file_or_files = if file_count == 1 { "file" } else { "files" };

        error!(
            "{}",
            style(format!("{file_count} unformatted {file_or_files}"))
                .bold()
                .red()
        );
    }
}
