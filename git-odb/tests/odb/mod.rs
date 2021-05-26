pub use git_testtools::{fixture_path, hex_to_id, scripted_fixture_repo_read_only};

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod alternate;
pub mod store;
