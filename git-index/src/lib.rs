#![deny(unsafe_code, missing_docs, rust_2018_idioms)]
#![allow(missing_docs, unused)]

use std::path::PathBuf;

use filetime::FileTime;

pub mod file;

pub mod extension {
    use crate::{util::read_u32, Version};

    const MIN_SIZE: usize = 4 /* signature */ + 4 /* size */;

    fn decode_header(data: &[u8]) -> ([u8; 4], u32, &[u8]) {
        let (signature, data) = data.split_at(4);
        let (size, data) = data.split_at(4);
        (signature.try_into().unwrap(), read_u32(size), data)
    }

    mod end_of_index_entry {
        use crate::{extension, extension::EndOfIndexEntry, file::header, util::read_u32};

        impl EndOfIndexEntry {
            pub fn from_bytes(data: &[u8], object_hash: git_hash::Kind) -> Option<Self> {
                let hash_len = object_hash.len_in_bytes();
                if data.len() < EndOfIndexEntry::SIZE_WITH_HEADER + hash_len {
                    return None;
                }

                let start_of_eoie = data.len() - EndOfIndexEntry::SIZE_WITH_HEADER - hash_len;
                let data = &data[start_of_eoie..][..hash_len];

                let (signature, ext_size, data) = extension::decode_header(data);
                if &signature != EndOfIndexEntry::SIGNATURE || ext_size as usize != EndOfIndexEntry::SIZE {
                    return None;
                }

                let (offset, hash) = data.split_at(4);
                let offset = read_u32(offset) as usize;
                if offset < header::SIZE {
                    return None;
                }
                todo!("eoie")
            }
        }
    }

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
    use filetime::FileTime;

    use crate::State;

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

pub(crate) mod util {
    #[inline]
    pub fn read_u32(b: &[u8]) -> u32 {
        u32::from_be_bytes(b.try_into().unwrap())
    }
}
