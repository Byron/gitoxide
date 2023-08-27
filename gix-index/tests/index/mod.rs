use std::path::{Path, PathBuf};

use gix_hash::ObjectId;

mod access;
mod entry;
mod file;
mod init;

pub fn hex_to_id(hex: &str) -> ObjectId {
    ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

pub fn fixture_index_path(name: &str) -> PathBuf {
    let dir =
        gix_testtools::scripted_fixture_read_only_standalone(Path::new("make_index").join(name).with_extension("sh"))
            .expect("script works");
    dir.join(".git").join("index")
}

pub fn loose_file_path(name: &str) -> PathBuf {
    gix_testtools::fixture_path_standalone(Path::new("loose_index").join(name).with_extension("git-index"))
}

#[test]
fn size_of_entry() {
    assert_eq!(std::mem::size_of::<gix_index::Entry>(), 80);

    // the reason we have our own time is half the size.
    assert_eq!(std::mem::size_of::<gix_index::entry::stat::Time>(), 8);
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

    pub fn open(&self) -> gix_index::File {
        gix_index::File::at(self.to_path(), gix_hash::Kind::Sha1, false, Default::default())
            .expect("fixtures are always readable")
    }
}
