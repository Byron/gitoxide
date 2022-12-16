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
        if let Some(link) = file.link.take() {
            dissolve_link_extension(&mut file, link, object_hash, options)?;
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

fn dissolve_link_extension(
    split_index: &mut File,
    link: extension::Link,
    object_hash: git_hash::Kind,
    options: decode::Options,
) -> Result<(), Error> {
    let shared_index_path = split_index
        .path
        .parent()
        .expect("split index file in .git folder")
        .join(format!("sharedindex.{}", link.shared_index_checksum));
    let mut shared_index = File::at(&shared_index_path, object_hash, options)?;

    if let Some(bitmaps) = &link.bitmaps {
        let mut counter = 0;
        let mut shared_entries = shared_index.entries_mut();
        let split_entries = split_index.entries();

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
                let start = shared_index.path_backing.len();
                e.path = start..start + split_entry.path.len();
                shared_index.entries.push(e);

                shared_index
                    .path_backing
                    .extend_from_slice(&split_index.path_backing[split_entry.path.clone()]);
            });
        }

        let mut removed_count = 0;
        bitmaps.delete.for_each_set_bit(|index| {
            shared_index.entries.remove(index - removed_count);
            removed_count += 1;
            Some(())
        });

        let mut shared_entries = std::mem::take(&mut shared_index.entries);
        shared_entries.sort_by(|a, b| a.cmp(b, &shared_index.state));

        std::mem::swap(&mut split_index.entries, &mut shared_entries);
        std::mem::swap(&mut split_index.path_backing, &mut shared_index.path_backing);
    }

    Ok(())
}
