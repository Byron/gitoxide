use std::path::{Path, PathBuf};

mod file;

pub fn fixture_path(name: &str) -> PathBuf {
    let dir = git_testtools::scripted_fixture_repo_read_only(Path::new("make_index").join(name).with_extension("sh"))
        .expect("script works");
    dir.join(".git").join("index")
}

#[test]
fn size_of_entry() {
    assert_eq!(std::mem::size_of::<git_index::Entry>(), 80);

    // the reason we have our own time is half the size.
    assert_eq!(std::mem::size_of::<git_index::entry::Time>(), 8);
    assert_eq!(std::mem::size_of::<filetime::FileTime>(), 16);
}
