use std::path::PathBuf;

mod borrowed;
mod owned;

pub fn hex_to_id(hex: &str) -> git_object::owned::Id {
    git_object::owned::Id::from_40_bytes_in_hex(hex.as_bytes()).unwrap()
}

pub fn fixture(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}

fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(fixture(path)).unwrap()
}
