use git_object::{borrowed, HashKind};
use std::path::PathBuf;

pub struct Db {
    pub path: PathBuf,
}

/// Initialization
impl Db {
    pub fn at(path: impl Into<PathBuf>) -> Db {
        Db { path: path.into() }
    }
}

pub(crate) fn sha1_path(id: borrowed::Id, mut root: PathBuf) -> PathBuf {
    match id.kind() {
        HashKind::Sha1 => {
            let hex = id.to_sha1_hex();
            let buf = std::str::from_utf8(&hex).expect("ascii only in hex");
            root.push(&buf[..2]);
            root.push(&buf[2..]);
            root
        }
    }
}

pub mod iter;
pub mod locate;
pub mod write;
