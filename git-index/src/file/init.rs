#![allow(unused)]

use std::path::{Path, PathBuf};

use memmap2::Mmap;

use crate::{decode, extension, File, State};

mod error {

    /// The error returned by [File::at()][super::File::at()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("An IO error occurred while opening the index")]
        Io(#[from] std::io::Error),
        #[error(transparent)]
        Decode(#[from] crate::decode::Error),
    }
}

pub use error::Error;

/// Initialization
impl File {
    /// Open an index file at `path` with `options`.
    pub fn at(path: impl Into<PathBuf>, options: decode::Options) -> Result<Self, Error> {
        let path = path.into();
        let (data, mtime) = {
            // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
            let file = std::fs::File::open(&path)?;
            #[allow(unsafe_code)]
            let data = unsafe { Mmap::map(&file)? };
            (data, filetime::FileTime::from_last_modification_time(&file.metadata()?))
        };

        let (state, checksum) = State::from_bytes(&data, mtime, options)?;
        Ok(File { state, path, checksum })
    }
}
