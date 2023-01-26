pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use git_hash::hex_to_id;

mod blob;
mod tree;
