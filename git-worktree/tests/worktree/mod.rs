mod fs;
mod index;

use std::path::{Path, PathBuf};
pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn fixture_path(name: &str) -> PathBuf {
    let dir = git_testtools::scripted_fixture_read_only(Path::new(name).with_extension("sh")).expect("script works");
    dir
}
