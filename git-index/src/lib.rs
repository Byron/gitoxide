#![deny(unsafe_code, missing_docs, rust_2018_idioms)]
#![allow(missing_docs, unused)]

use std::path::PathBuf;

use filetime::FileTime;

pub mod file;

pub(crate) mod extension;

mod access {
    use crate::{State, Version};

    impl State {
        pub fn version(&self) -> Version {
            self.version
        }
    }
}

pub mod init {
    use filetime::FileTime;

    use crate::{State, Version};

    impl State {
        /// Returns an empty state.
        /// TODO: figure out if it needs to know some configuration, and if this would actually be used somewhere
        fn new() -> Self {
            State {
                timestamp: FileTime::from_system_time(std::time::SystemTime::UNIX_EPOCH),
                version: Version::V3,
                cache_tree: None,
            }
        }
    }

    impl Default for State {
        fn default() -> Self {
            State::new()
        }
    }
}

pub mod decode;

/// All known versions of a git index file.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Version {
    V2 = 2,
    V3 = 3,
    V4 = 4,
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
pub struct State {
    /// The time at which the state was created, indicating its freshness compared to other files on disk.
    ///
    /// Note that on platforms that only have a precisions of a second for this time, we will treat all entries with the
    /// same timestamp as this as potentially changed, checking more thoroughly if a change actually happened.
    timestamp: FileTime,
    version: Version,
    pub cache_tree: Option<extension::Tree>,
}

pub(crate) mod util {
    #[inline]
    pub fn read_u32(b: &[u8]) -> u32 {
        u32::from_be_bytes(b.try_into().unwrap())
    }

    #[inline]
    pub fn split_at_byte_exclusive(data: &[u8], byte: u8) -> Option<(&[u8], &[u8])> {
        if data.len() < 2 {
            return None;
        }
        data.iter().enumerate().find_map(|(idx, b)| {
            (*b == byte).then(|| {
                if idx == 0 {
                    (&[] as &[u8], &data[1..])
                } else {
                    let (a, b) = data.split_at(idx);
                    (a, &b[1..])
                }
            })
        })
    }
}
