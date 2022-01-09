#![deny(unsafe_code, missing_docs, rust_2018_idioms)]
#![allow(missing_docs)]

use std::path::PathBuf;

pub mod file {
    pub mod init {
        #![allow(unused)]
        use crate::File;
        use memmap2::Mmap;
        use std::path::Path;

        mod error {
            use quick_error::quick_error;

            quick_error! {
                #[derive(Debug)]
                pub enum Error {
                    Io(err: std::io::Error) {
                        display("An IO error occurred while reading the index")
                        source(err)
                        from()
                    }
                }
            }
        }
        pub use error::Error;

        impl File {
            pub fn at(path: impl AsRef<Path>, object_hash: git_hash::Kind) -> Result<Self, Error> {
                // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
                #[allow(unsafe_code)]
                let data = unsafe { Mmap::map(&std::fs::File::open(path)?)? };

                todo!("read file")
            }
        }
    }
}
pub mod init {
    use crate::State;

    impl State {
        /// Returns an empty state.
        /// TODO: figure out if it needs to know some configuration
        pub fn new() -> Self {
            State
        }
    }

    impl Default for State {
        fn default() -> Self {
            State::new()
        }
    }
}

/// An index file whose state was read from a file on disk.
pub struct File {
    pub state: State,
    pub path: PathBuf,
}

/// An in-memory cache of a fully parsed git index file.
///
/// As opposed to a snapshot, it's meant to be altered and eventually be written back to disk or converted into a tree.
/// We treat index and its state synonymous.
pub struct State;
