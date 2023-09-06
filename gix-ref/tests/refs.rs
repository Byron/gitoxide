use gix_hash::ObjectId;

pub fn hex_to_id(hex: &str) -> ObjectId {
    ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

pub use gix_testtools::Result;

mod file;
mod fullname;
mod namespace;
mod packed;
mod reference;
mod store;
mod transaction;
