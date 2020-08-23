use std::path::PathBuf;

pub type Error = Box<dyn std::error::Error>;
pub type Result = std::result::Result<(), Error>;

pub fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path)
}
pub fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(fixture_path(path)).expect("fixture to be present and readable")
}

mod client;
