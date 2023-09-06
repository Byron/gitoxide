use std::path::PathBuf;

use crate::store::WriteReflog;

mod error {
    /// The error returned by [`crate::Store::at()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("There was an error accessing the store's directory")]
        Io(#[from] std::io::Error),
    }
}

pub use error::Error;

use crate::file;

#[allow(dead_code)]
impl crate::Store {
    /// Create a new store at the given location, typically the `.git/` directory.
    ///
    /// `object_hash` defines the kind of hash to assume when dealing with refs.
    pub fn at(git_dir: PathBuf, reflog_mode: WriteReflog, object_hash: gix_hash::Kind) -> Result<Self, Error> {
        // for now, just try to read the directory - later we will do that naturally as we have to figure out if it's a ref-table or not.
        std::fs::read_dir(&git_dir)?;
        Ok(crate::Store {
            inner: crate::store::State::Loose {
                store: file::Store::at(git_dir, reflog_mode, object_hash),
            },
        })
    }
}
