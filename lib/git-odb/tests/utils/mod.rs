use hex::FromHex;
use std::path::PathBuf;

pub fn bin(hex: &str) -> [u8; 20] {
    <[u8; 20]>::from_hex(hex).unwrap()
}

pub fn fixture(path: &str) -> PathBuf {
    let mut b = PathBuf::from(file!());
    b.pop();
    b.pop();
    b.push("fixtures");
    b.push(path);
    b
}
