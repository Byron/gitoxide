use std::path::PathBuf;

use gix_hash::ObjectId;

mod encode;
mod immutable;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(not(windows))]
fn fixup(v: Vec<u8>) -> Vec<u8> {
    v
}

#[cfg(windows)]
fn fixup(v: Vec<u8>) -> Vec<u8> {
    // Git checks out text files with line ending conversions, git itself will of course not put '\r\n' anywhere,
    // so that wouldn't be expected in an object and doesn't have to be parsed.
    use bstr::ByteSlice;
    v.replace(b"\r\n", "\n")
}

pub fn fixture(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}

fn fixture_bytes(path: &str) -> Vec<u8> {
    fixup(std::fs::read(fixture(path)).unwrap())
}

#[test]
fn size_in_memory() {
    let actual = std::mem::size_of::<gix_object::Object>();
    assert!(
        actual <= 264,
        "{actual} <= 264: Prevent unexpected growth of what should be lightweight objects"
    )
}

fn hex_to_id(hex: &str) -> ObjectId {
    ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}
