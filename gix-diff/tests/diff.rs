pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn hex_to_id(hex: &str) -> gix_hash::ObjectId {
    gix_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

mod blob;
mod tree;
