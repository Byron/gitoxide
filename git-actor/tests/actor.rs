use std::path::PathBuf;

pub use git_hash::hex_to_id;

pub fn fixture(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}

mod signature;
