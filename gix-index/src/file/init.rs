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
    /// index that merely exists in memory and is empty. `skip_hash` will increase the performance by a factor of 2, at the cost of
    /// possibly not detecting corruption.
    ///
    /// Note that the `path` will not be written if it doesn't exist.
    pub fn at_or_default(
        path: impl Into<PathBuf>,
        object_hash: gix_hash::Kind,
        skip_hash: bool,
        options: decode::Options,
    ) -> Result<Self, Error> {
        let path = path.into();
        Ok(match Self::at(&path, object_hash, skip_hash, options) {
            Ok(f) => f,
            Err(Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {
                File::from_state(State::new(object_hash), path)
            }
            Err(err) => return Err(err),
        })
    }

    /// Open an index file at `path` with `options`, assuming `object_hash` is used throughout the file. If `skip_hash` is `true`,
    /// we will not get or compare the checksum of the index at all, which generally increases performance of this method by a factor
    /// of 2 or more.
    ///
    /// Note that the verification of the file hash depends on `options`, and even then it's performed after the file was read and not
    /// before it is read. That way, invalid files would see a more descriptive error message as we try to parse them.
    pub fn at(
        path: impl Into<PathBuf>,
        object_hash: gix_hash::Kind,
        skip_hash: bool,
        options: decode::Options,
    ) -> Result<Self, Error> {
        let _span = gix_features::trace::detail!("gix_index::File::at()");
        let path = path.into();
        let (data, mtime) = {
            let file = std::fs::File::open(&path)?;
            // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
            #[allow(unsafe_code)]
            let data = unsafe { Mmap::map(&file)? };

            if !skip_hash {
                let expected = gix_hash::ObjectId::from(&data[data.len() - object_hash.len_in_bytes()..]);
                if !expected.is_null() {
                    let _span = gix_features::trace::detail!("gix::open_index::hash_index", path = ?path);
                    let meta = file.metadata()?;
                    let num_bytes_to_hash = meta.len() - object_hash.len_in_bytes() as u64;
                    let actual_hash = gix_features::hash::bytes(
                        &file,
                        num_bytes_to_hash as usize,
                        object_hash,
                        &mut gix_features::progress::Discard,
                        &Default::default(),
                    )?;

                    if actual_hash != expected {
                        return Err(Error::Decode(decode::Error::ChecksumMismatch {
                            actual_checksum: actual_hash,
                            expected_checksum: expected,
                        }));
                    }
                }
            }

            (data, filetime::FileTime::from_last_modification_time(&file.metadata()?))
        };

        let (state, checksum) = State::from_bytes(&data, mtime, object_hash, options)?;
        let mut file = File { state, path, checksum };
        if let Some(mut link) = file.link.take() {
            link.dissolve_into(&mut file, object_hash, skip_hash, options)?;
        }

        Ok(file)
    }

    /// Consume `state` and pretend it was read from `path`, setting our checksum to `null`.
    ///
    /// `File` instances created like that should be written to disk to set the correct checksum via `[File::write()]`.
    pub fn from_state(state: State, path: impl Into<PathBuf>) -> Self {
        File {
            state,
            path: path.into(),
            checksum: None,
        }
    }
}
