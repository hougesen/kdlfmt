use std::iter::Flatten;

const KDLFMT_IGNORE_FILE_NAME: &str = ".kdlfmtignore";

pub const KDL_FILE_EXTENSION: &str = "kdl";

#[inline]
pub fn read_file(path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

#[inline]
pub fn save_file(path: &std::path::Path, contents: &str) -> std::io::Result<()> {
    std::fs::write(path, contents)
}

#[inline]
pub fn setup_walker(mut paths: Vec<std::path::PathBuf>) -> Flatten<ignore::Walk> {
    let path = paths.pop().unwrap_or_else(|| std::path::PathBuf::from("."));

    let mut walker = ignore::WalkBuilder::new(path);

    for path in paths {
        walker.add(path);
    }

    walker
        .standard_filters(true)
        .parents(true)
        .hidden(true)
        .add_custom_ignore_filename(KDLFMT_IGNORE_FILE_NAME)
        .build()
        .flatten()
}
