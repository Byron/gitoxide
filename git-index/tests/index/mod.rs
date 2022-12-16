use std::path::{Path, PathBuf};

mod file;
mod init;

pub fn fixture_index_path(name: &str) -> PathBuf {
    let dir = git_testtools::scripted_fixture_read_only(Path::new("make_index").join(name).with_extension("sh"))
        .expect("script works");
    dir.join(".git").join("index")
}

pub fn loose_file_path(name: &str) -> PathBuf {
    git_testtools::fixture_path(Path::new("loose_index").join(name).with_extension("git-index"))
}

#[test]
fn size_of_entry() {
    assert_eq!(std::mem::size_of::<git_index::Entry>(), 80);

    // the reason we have our own time is half the size.
    assert_eq!(std::mem::size_of::<git_index::entry::Time>(), 8);
    assert_eq!(std::mem::size_of::<filetime::FileTime>(), 16);
}

enum Fixture {
    Generated(&'static str),
    Loose(&'static str),
}

impl Fixture {
    pub fn to_path(&self) -> PathBuf {
        match self {
            Fixture::Generated(name) => fixture_index_path(name),
            Fixture::Loose(name) => loose_file_path(name),
        }
    }
    pub fn to_name(&self) -> &'static str {
        match self {
            Fixture::Generated(name) | Fixture::Loose(name) => name,
        }
    }
}
