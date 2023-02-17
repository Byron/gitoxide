use gix_hash::ObjectId;
pub use gix_testtools::{fixture_path, scripted_fixture_read_only};

pub fn hex_to_id(hex: &str) -> ObjectId {
    ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn db() -> gix_odb::Handle {
    gix_odb::at(fixture_path("objects")).expect("valid object path")
}

fn db_small_packs() -> gix_odb::Handle {
    gix_odb::at(fixture_path("repos/small-packs.git/objects")).unwrap()
}

pub mod alternate;
pub mod find;
pub mod header;
pub mod regression;
pub mod sink;
pub mod store;
