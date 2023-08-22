mod checkout;

use std::path::{Path, PathBuf};

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn fixture_path(name: &str) -> PathBuf {
    let dir = gix_testtools::scripted_fixture_read_only_standalone(Path::new(name).with_extension("sh"))
        .expect("script works");
    dir
}
