mod fs;
mod index;

use std::path::{Path, PathBuf};

use gix_hash::ObjectId;
pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn hex_to_id(hex: &str) -> ObjectId {
    ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

pub fn fixture_path(name: &str) -> PathBuf {
    let dir = gix_testtools::scripted_fixture_read_only(Path::new(name).with_extension("sh")).expect("script works");
    dir
}
