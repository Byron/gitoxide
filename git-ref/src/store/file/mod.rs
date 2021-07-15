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

/// A git _ref_ which is stored in a file.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Reference<'a> {
    parent: &'a Store,
    /// The path to uniquely identify this ref within its store.
    relative_path: PathBuf,
    state: reference::State,
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

mod traits {
    use crate::{
        mutable::Target,
        store::{file, file::find_one},
        PartialName,
    };

    impl crate::traits::RefStore for file::Store {
        type FindOneExistingError = find_one::existing::Error;

        fn find_one_existing(&self, name: PartialName<'_>) -> Result<Target, Self::FindOneExistingError> {
            self.find_one_existing(name).map(|r| r.into_target())
        }
    }
}

mod loose;
pub use loose::{find_one, iter};

mod packed {
    use crate::store::{file, packed};
    use std::path::PathBuf;

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

///
pub mod reference;

///
pub mod log;

///
pub mod transaction;
