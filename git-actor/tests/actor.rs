use std::path::PathBuf;

mod signature;

pub use git_testtools::hex_to_id;

pub fn fixture(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}
