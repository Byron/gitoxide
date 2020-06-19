use hex::FromHex;
use std::path::PathBuf;

mod borrowed;

pub fn hex_to_id(hex: &str) -> [u8; 20] {
    <[u8; 20]>::from_hex(hex).unwrap()
}

pub fn fixture(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}

fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(fixture(path)).unwrap()
}
