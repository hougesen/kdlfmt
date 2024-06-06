use console::style;

use crate::error::KdlFmtError;

#[inline]
pub fn print_error(error: &KdlFmtError) {
    eprintln!("{error}");
}

#[inline]
pub fn print_format_changed_file(path: &std::path::Path, duration: core::time::Duration) {
    println!("{} finished in {}ms", path.display(), duration.as_millis());
}

#[inline]
pub fn print_format_unchanged_file(path: &std::path::Path, duration: core::time::Duration) {
    println!(
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

    println!("{file_count } files was formatted in {time_taken}");
}
