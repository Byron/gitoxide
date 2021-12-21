//! An object database storing each object in a zlib compressed file with its hash in the path
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 512;
use git_features::fs;
use std::path::PathBuf;

/// A database for reading and writing objects to disk, one file per object.
#[derive(Clone, PartialEq, Eq)]
pub struct Store {
    /// The directory in which objects are stored, containing 256 folders representing the hashes first byte.
    pub path: PathBuf,
}

/// Initialization
impl Store {
    /// Initialize the Db with the `objects_directory` containing the hexadecimal first byte subdirectories, which in turn
    /// contain all loose objects.
    ///
    /// In a git repository, this would be `.git/objects`.
    pub fn at(objects_directory: impl Into<PathBuf>) -> Store {
        Store {
            path: objects_directory.into(),
        }
    }
}

fn hash_path(id: &git_hash::oid, mut root: PathBuf) -> PathBuf {
    let mut hex = git_hash::Kind::hex_buf();
    id.hex_to_buf(hex.as_mut());
    let buf = std::str::from_utf8(&hex).expect("ascii only in hex");
    root.push(&buf[..2]);
    root.push(&buf[2..]);
    root
}

///
pub mod find;
///
pub mod iter;

/// The type for an iterator over `Result<git_hash::ObjectId, Error>)`
pub type Iter = std::iter::FilterMap<
    fs::walkdir::DirEntryIter,
    fn(Result<fs::walkdir::DirEntry, fs::walkdir::Error>) -> Option<Result<git_hash::ObjectId, iter::Error>>,
>;

///
pub mod write;
