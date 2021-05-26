pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use git_testtools::hex_to_id;

mod visit;
