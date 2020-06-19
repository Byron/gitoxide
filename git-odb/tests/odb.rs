use hex::FromHex;
use std::path::PathBuf;

pub fn bin(hex: &str) -> [u8; 20] {
    <[u8; 20]>::from_hex(hex).unwrap()
}

pub fn fixture(path: &str) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path)
}

mod loose;
mod pack;
