#[cfg(feature = "describe")]
mod describe;
mod spec;
pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

fn hex_to_id(hex: &str) -> gix_hash::ObjectId {
    gix_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}
