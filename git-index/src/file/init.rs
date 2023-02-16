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
        #[error(transparent)]
        LinkExtension(#[from] crate::extension::link::decode::Error),
    }
}

pub use error::Error;

/// Initialization
impl File {
    /// Try to open the index file at `path` with `options`, assuming `object_hash` is used throughout the file, or create a new
    /// index that merely exists in memory and is empty.
    ///
    /// Note that the `path` will not be written if it doesn't exist.
    pub fn at_or_default(
        path: impl Into<PathBuf>,
        object_hash: gix_hash::Kind,
        options: decode::Options,
    ) -> Result<Self, Error> {
        let path = path.into();
        Ok(match Self::at(&path, object_hash, options) {
            Ok(f) => f,
            Err(Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {
                File::from_state(State::new(object_hash), path)
            }
            Err(err) => return Err(err),
        })
    }

    /// Open an index file at `path` with `options`, assuming `object_hash` is used throughout the file.
    pub fn at(path: impl Into<PathBuf>, object_hash: gix_hash::Kind, options: decode::Options) -> Result<Self, Error> {
        let path = path.into();
        let (data, mtime) = {
            // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
            let file = std::fs::File::open(&path)?;
            #[allow(unsafe_code)]
            let data = unsafe { Mmap::map(&file)? };
            (data, filetime::FileTime::from_last_modification_time(&file.metadata()?))
        };

        let (state, checksum) = State::from_bytes(&data, mtime, object_hash, options)?;
        let mut file = File {
            state,
            path,
            checksum: Some(checksum),
        };
        if let Some(mut link) = file.link.take() {
            link.dissolve_into(&mut file, object_hash, options)?;
        }

        Ok(file)
    }

    /// Consume `state` and pretend it was read from `path`, setting our checksum to `null`.
    ///
    /// `File` instances created like that should be written to disk to set the correct checksum via `[File::write()]`.
    pub fn from_state(state: crate::State, path: impl Into<PathBuf>) -> Self {
        File {
            state,
            path: path.into(),
            checksum: None,
        }
    }
}
