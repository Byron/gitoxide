use crate::store::WriteReflog;
use std::convert::TryFrom;
use std::path::PathBuf;

mod error {
    use quick_error::quick_error;

    quick_error! {
        /// The error returned by [crate::Store::at()].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            Io(err: std::io::Error) {
                display("There was an error accessing the store's directory")
                from()
                source(err)
            }
        }
    }
}

pub use error::Error;

impl crate::Store {
    /// Create a new store at the given location, typically the `.git/` directory.
    pub fn at(git_dir: impl Into<PathBuf>, reflog_mode: WriteReflog) -> Result<Self, Error> {
        // for now, just try to read the directory - later we will do that naturally as we have to figure out if it's a ref-table or not.
        let git_dir = git_dir.into();
        std::fs::read_dir(&git_dir)?;
        Ok(crate::Store {
            _path: git_dir,
            _reflog_mode: reflog_mode,
        })
    }
}

impl TryFrom<PathBuf> for crate::Store {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        crate::Store::at(value, WriteReflog::Normal)
    }
}
