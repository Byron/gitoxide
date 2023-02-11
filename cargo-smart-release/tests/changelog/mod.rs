use gix::ObjectId;

mod parse;

mod write_and_parse;

mod merge;

fn hex_to_id(hex: &str) -> ObjectId {
    ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}
