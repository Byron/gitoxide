use hex::FromHex;
use std::path::PathBuf;
use std::{fs::File, io::Read};

pub fn fixture(path: &str) -> PathBuf {
    let mut b = PathBuf::from(file!());
    b.pop();
    b.pop();
    b.push("fixtures");
    b.push(path);
    b
}

pub fn fixture_bytes(path: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    File::open(fixture(path))
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    buf
}

pub fn bin(hex: &str) -> [u8; 20] {
    <[u8; 20]>::from_hex(hex).unwrap()
}
