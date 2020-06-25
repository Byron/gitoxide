use std::path::PathBuf;

pub fn hex_to_id(hex: &str) -> git_object::Id {
    git_object::Id::from_hex(hex.as_bytes()).unwrap()
}

pub fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path)
}

mod loose;
mod pack;
