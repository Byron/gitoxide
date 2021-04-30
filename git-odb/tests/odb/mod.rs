pub use git_testtools::{fixture_path, scripted_fixture_repo_read_only};

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(not(windows))]
pub fn fixup(v: Vec<u8>) -> Vec<u8> {
    v
}

#[cfg(windows)]
pub fn fixup(v: Vec<u8>) -> Vec<u8> {
    // Git checks out text files with line ending conversions, git itself will of course not put '\r\n' anywhere,
    // so that wouldn't be expected in an object and doesn't have to be parsed.
    use bstr::ByteSlice;
    v.replace(b"\r\n", "\n")
}

pub fn hex_to_id(hex: &str) -> git_hash::ObjectId {
    git_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

pub mod alternate;
pub mod compound;
pub mod linked;
pub mod loose;
pub mod pack;
pub mod sink;
