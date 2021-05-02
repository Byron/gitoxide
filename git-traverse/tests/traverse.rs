pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn hex_to_id(hex: &str) -> git_hash::ObjectId {
    git_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

mod commit;
