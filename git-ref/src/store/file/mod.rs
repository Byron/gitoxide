use std::path::PathBuf;

/// The way a file store handles the reflog
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
pub enum WriteReflog {
    /// Write a ref log for ref edits according to the standard rules.
    Normal,
    /// Never write a ref log.
    Disable,
}

impl Default for WriteReflog {
    fn default() -> Self {
        WriteReflog::Normal
    }
}

/// A store for reference which uses plain files.
///
/// Each ref is represented as a single file on disk in a folder structure that follows the relative path
/// used to identify [references][Reference].
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Store {
    /// The location at which loose references can be found as per conventions of a typical git repository.
    ///
    /// Typical base paths are `.git` repository folders.
    pub base: PathBuf,
    /// The way to handle reflog edits
    pub write_reflog: WriteReflog,
}

pub(in crate::store::file) fn path_to_name(path: impl Into<PathBuf>) -> bstr::BString {
    use os_str_bytes::OsStringBytes;
    let path = path.into().into_raw_vec();
    #[cfg(windows)]
    let path = {
        use bstr::ByteSlice;
        path.replace(b"\\", b"/")
    };
    path.into()
}

///
pub mod loose;
mod overlay;
pub use overlay::Reference;

///
pub mod iter {
    pub use super::{
        loose::iter::{loose, Loose},
        overlay::LooseThenPacked,
    };
    ///
    pub mod loose_then_packed {
        pub use super::super::overlay::iter::Error;
    }
}

///
pub mod log;

///
pub mod find;
///
pub mod transaction;

mod packed {
    use std::path::PathBuf;

    use crate::store::{file, packed};

    impl file::Store {
        /// Return a buffer for the packed file
        pub fn packed(&self) -> Result<Option<packed::Buffer>, packed::buffer::open::Error> {
            match packed::Buffer::open(self.packed_refs_path(), 32 * 1024) {
                Ok(buf) => Ok(Some(buf)),
                Err(packed::buffer::open::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(err) => Err(err),
            }
        }

        /// Return the path at which packed-refs would usually be stored
        pub fn packed_refs_path(&self) -> PathBuf {
            self.base.join("packed-refs")
        }
    }
}
