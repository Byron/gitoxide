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
                version: Version::V4,
            }
        }
    }

    impl Default for State {
        fn default() -> Self {
            State::new()
        }
    }
}

pub mod decode {
    use crate::{extension, header, State};
    use filetime::FileTime;

    mod error {
        use quick_error::quick_error;

        use crate::header;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Header(err: header::decode::Error) {
                    display("The header could not be decoded")
                    source(err)
                    from()
                }
            }
        }
    }
    pub use error::Error;

    impl State {
        pub fn from_bytes(data: &[u8], timestamp: FileTime, object_hash: git_hash::Kind) -> Result<Self, Error> {
            let (version, num_entries, post_header_data) = header::decode(&data, object_hash)?;
            let start_of_extensions = extension::end_of_index_entry::decode(&data, object_hash);

            Ok(State { timestamp, version })
        }
    }
}

pub mod header {
    pub(crate) const SIZE: usize = 4 /*signature*/ + 4 /*version*/ + 4 /* num entries */;

    pub mod decode {
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Corrupt(message: &'static str) {
                    display("{}", message)
                }
                UnsupportedVersion(version: u32) {
                    display("Index version {} is not supported", version)
                }
            }
        }
    }
    use crate::{util::read_u32, Version};

    pub(crate) fn decode(
        data: &[u8],
        object_hash: git_hash::Kind,
    ) -> Result<(crate::Version, u32, &[u8]), decode::Error> {
        if data.len() < (3 * 4) + object_hash.len_in_bytes() {
            return Err(decode::Error::Corrupt(
                "File is too small even for header with zero entries and smallest hash",
            ));
        }

        const SIGNATURE: &[u8] = b"DIRC";
        let (signature, data) = data.split_at(4);
        if signature != SIGNATURE {
            return Err(decode::Error::Corrupt(
                "Signature mismatch - this doesn't claim to be a header file",
            ));
        }

        let (version, data) = data.split_at(4);
        let version = match read_u32(version) {
            2 => Version::V2,
            3 => Version::V3,
            4 => Version::V4,
            unknown => return Err(decode::Error::UnsupportedVersion(unknown)),
        };
        let (entries, data) = data.split_at(4);
        let entries = read_u32(entries);

        Ok((version, entries, data))
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
    version: Version,
}

pub(crate) mod util {
    #[inline]
    pub fn read_u32(b: &[u8]) -> u32 {
        u32::from_be_bytes(b.try_into().unwrap())
    }
}
