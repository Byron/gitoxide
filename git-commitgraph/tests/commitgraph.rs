use git_object::owned;
use std::path::PathBuf;

mod access;

pub fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path)
}

pub fn hex_to_id(hex: &[u8]) -> owned::Id {
    owned::Id::from_40_bytes_in_hex(hex).expect("40 bytes hex")
}
