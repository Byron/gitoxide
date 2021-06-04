type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(std::path::PathBuf::from("tests").join("fixtures").join(path))
        .expect("fixture to be present and readable")
}

mod credentials;
#[cfg(feature = "blocking-client")]
mod fetch;
mod remote_progress;
