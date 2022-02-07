use std::path::{Path, PathBuf};

use walkdir::WalkDir;

mod copy_index;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn dir_structure<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let path = path.as_ref();
    let mut ps: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| e.path() == path || !e.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false))
        .flatten()
        .filter(|e| e.path().is_file())
        .map(|p| p.path().to_path_buf())
        .collect();
    ps.sort();
    ps
}

pub fn fixture_path(name: &str) -> PathBuf {
    let dir =
        git_testtools::scripted_fixture_repo_read_only(Path::new(name).with_extension("sh")).expect("script works");
    dir
}
