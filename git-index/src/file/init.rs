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
    /// Open an index file at `path` with `options`, assuming `object_hash` is used throughout the file.
    pub fn at(path: impl Into<PathBuf>, object_hash: git_hash::Kind, options: decode::Options) -> Result<Self, Error> {
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
        file.resolve_link_extension(object_hash, options);

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

    fn resolve_link_extension(&mut self, object_hash: git_hash::Kind, options: decode::Options) -> Result<(), Error> {
        if let Some(link) = self.link() {
            let shared_index_path = self
                .path
                .parent()
                .expect("parent")
                .join(format!("sharedindex.{}", link.shared_index_checksum));
            let mut shared_index = File::at(&shared_index_path, object_hash, options)?;

            let shared_entries = shared_index.entries();
            let split_entries = self.entries();

            if let Some(bitmaps) = &link.bitmaps {
                let mut counter = 0;
                bitmaps.replace.for_each_set_bit(|index| {
                    println!("replace shared[{index}] with split[{counter}], but keep path");
                    counter += 1;
                    Some(())
                });

                if split_entries.len() > counter {
                    println!("add entries split[{}..{}] to shared", counter, split_entries.len());
                    split_entries[counter..].iter().for_each(|e| {
                        println!(
                            "  add entry, extend path backing with {:?}",
                            e.path_in(self.path_backing())
                        )
                    });
                }

                bitmaps.delete.for_each_set_bit(|index| {
                    println!("remove shared[{index}]");
                    Some(())
                });

                // TODO:
                //  - move merged entries into index.state.entries
                //  - probably sort entries
                //  - disable link extension
            }

            Ok(())
        } else {
            Ok(())
        }
    }
}
