#![allow(unused)]

use std::path::{Path, PathBuf};

use memmap2::Mmap;

use crate::{decode, extension, File, State};

mod error {
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Io(err: std::io::Error) {
                display("An IO error occurred while opening the index")
                source(err)
                from()
            }
            Decode(err: crate::decode::Error) {
                display("The file could not be decoded")
                source(err)
                from()
            }
        }
    }
}

pub use error::Error;

impl File {
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
