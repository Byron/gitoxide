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
            let mut shared_index = File::at(
                &self
                    .path
                    .parent()
                    .expect("parent")
                    .join(format!("sharedindex.{}", link.shared_index_checksum)),
                object_hash,
                options,
            )?;

            let mut shared_entries = shared_index.entries_mut();
            let split_entries = self.entries();

            if let Some(bitmaps) = &link.bitmaps {
                let mut counter = 0;
                bitmaps.replace.for_each_set_bit(|index| {
                    match (shared_entries.get_mut(index), split_entries.get(counter)) {
                        (Some(shared_entry), Some(split_entry)) => {
                            shared_entry.stat = split_entry.stat;
                            shared_entry.id = split_entry.id;
                            shared_entry.flags = split_entry.flags;
                            shared_entry.mode = split_entry.mode;
                        }
                        _ => unreachable!(),
                    }

                    counter += 1;
                    Some(())
                });

                if split_entries.len() > counter {
                    split_entries[counter..].iter().for_each(|split_entry| {
                        let mut e = split_entry.clone();
                        e.path =
                            shared_index.path_backing.len()..shared_index.path_backing.len() + split_entry.path.len();
                        shared_index.entries.push(e);

                        shared_index
                            .path_backing
                            .extend_from_slice(&self.path_backing[split_entry.path.clone()]);
                    });
                }

                let mut removed_count = 0;
                bitmaps.delete.for_each_set_bit(|index| {
                    shared_index.entries.remove(index - removed_count);
                    removed_count += 1;

                    Some(())
                });

                let mut entries = std::mem::take(&mut shared_index.entries);
                entries.sort_by(|a, b| a.cmp(b, &shared_index.state));

                std::mem::swap(&mut self.entries, &mut entries);
                std::mem::swap(&mut self.path_backing, &mut shared_index.path_backing);

                self.link = None
            }

            Ok(())
        } else {
            Ok(())
        }
    }
}
