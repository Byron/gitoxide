//! An object database storing each object in a zlib compressed file with its hash in the path
/// The maximum size that an object header can have. `git2` says 64, and `git` says 32 but also mentions it can be larger.
const HEADER_MAX_SIZE: usize = 64;
use std::path::{Path, PathBuf};

use git_features::fs;

/// A database for reading and writing objects to disk, one file per object.
#[derive(Clone, PartialEq, Eq)]
pub struct Store {
    /// The directory in which objects are stored, containing 256 folders representing the hashes first byte.
    pub(crate) path: PathBuf,
    /// The kind of hash we should assume during iteration and when writing new objects.
    pub(crate) object_hash: git_hash::Kind,
}

/// Initialization
impl Store {
    /// Initialize the Db with the `objects_directory` containing the hexadecimal first byte subdirectories, which in turn
    /// contain all loose objects.
    ///
    /// In a git repository, this would be `.git/objects`.
    ///
    /// The `object_hash` determines which hash to use when writing, finding or iterating objects.
    pub fn at(objects_directory: impl Into<PathBuf>, object_hash: git_hash::Kind) -> Store {
        Store {
            path: objects_directory.into(),
            object_hash,
        }
    }

    /// Return the path to our `objects` directory.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Return the kind of hash we would iterate and write.
    pub fn object_hash(&self) -> git_hash::Kind {
        self.object_hash
    }
}

fn hash_path(id: &git_hash::oid, mut root: PathBuf) -> PathBuf {
    let mut hex = git_hash::Kind::hex_buf();
    let hex_len = id.hex_to_buf(hex.as_mut());
    let buf = std::str::from_utf8(&hex[..hex_len]).expect("ascii only in hex");
    root.push(&buf[..2]);
    root.push(&buf[2..]);
    root
}

///
pub mod find;
///
pub mod iter;
///
pub mod verify;

/// The type for an iterator over `Result<git_hash::ObjectId, Error>)`
pub struct Iter {
    inner: fs::walkdir::DirEntryIter,
    hash_hex_len: usize,
}

///
pub mod write;
