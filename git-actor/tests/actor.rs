use std::path::PathBuf;

mod mutable;

pub use git_testtools::hex_to_id;

pub fn fixture(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}
