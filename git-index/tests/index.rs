use std::path::{Path, PathBuf};

mod file;

pub fn index_fixture_path(name: &str) -> PathBuf {
    let dir = git_testtools::scripted_fixture_repo_read_only(Path::new("make_index").join(name).with_extension("sh"))
        .expect("script works");
    dir.join(".git").join("index")
}
