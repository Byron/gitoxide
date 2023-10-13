use std::path::{Path, PathBuf};

pub use gix_testtools::Result;

fn fixture_path(path: impl AsRef<Path>) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path.as_ref())
}

mod parse;
