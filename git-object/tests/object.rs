use std::path::PathBuf;

mod borrowed;

pub fn hex_to_id(hex: &str) -> git_object::Id {
    git_object::Id::from_hex(hex.as_bytes()).unwrap()
}

pub fn fixture(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}

fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(fixture(path)).unwrap()
}
