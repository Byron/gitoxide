#![deny(unsafe_code, missing_docs, rust_2018_idioms)]
#![allow(missing_docs, unused)]

use filetime::FileTime;
use std::path::PathBuf;

pub mod file;

pub mod extension {
    const MIN_SIZE: usize = 4 /* signature */ + 4 /* size */;

    pub struct EndOfIndexEntry {
        /// The offset the the beginning of all extensions, or the end of all entries.
        offset_to_extensions: u32,
        /// The SHA1 checksum over the signature and size of all extensions.
        checksum: git_hash::ObjectId,
    }

    impl EndOfIndexEntry {
        pub const SIGNATURE: &'static [u8] = b"EOIE";
        pub const SIZE: usize = 4 /* offset to extensions */ + git_hash::Kind::Sha1.len_in_bytes();
        pub const SIZE_WITH_HEADER: usize = crate::extension::MIN_SIZE + Self::SIZE;
    }
}

pub mod init {
    use crate::State;
    use filetime::FileTime;

    impl State {
        /// Returns an empty state.
        /// TODO: figure out if it needs to know some configuration, and if this would actually be used somewhere
        fn new() -> Self {
            State {
                timestamp: FileTime::from_system_time(std::time::SystemTime::UNIX_EPOCH),
            }
        }
    }

    impl Default for State {
        fn default() -> Self {
            State::new()
        }
    }
}

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
}
