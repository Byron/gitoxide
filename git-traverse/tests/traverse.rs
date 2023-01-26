pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

use git_hash::hex_to_id;

mod commit;
mod tree;
