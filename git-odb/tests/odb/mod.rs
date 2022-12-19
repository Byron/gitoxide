pub use git_testtools::{fixture_path, hex_to_id, scripted_fixture_read_only};

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn db() -> git_odb::Handle {
    git_odb::at(fixture_path("objects")).expect("valid object path")
}

fn db_small_packs() -> git_odb::Handle {
    git_odb::at(fixture_path("repos/small-packs.git/objects")).unwrap()
}

pub mod alternate;
pub mod find;
pub mod header;
pub mod regression;
pub mod sink;
pub mod store;
