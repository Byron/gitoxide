type Result = std::result::Result<(), Box<dyn std::error::Error>>;
use std::path::PathBuf;

pub fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(PathBuf::from("tests").join("fixtures").join(path)).expect("fixture to be present and readable")
}

mod credentials;
mod fetch;
mod remote_progress;
