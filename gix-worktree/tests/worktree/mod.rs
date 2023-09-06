use gix_hash::ObjectId;

mod stack;

pub use gix_testtools::Result;

pub fn hex_to_id(hex: &str) -> ObjectId {
    ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}
