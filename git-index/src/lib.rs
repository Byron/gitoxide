//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![deny(unsafe_code, missing_docs, rust_2018_idioms)]
#![allow(missing_docs)]

use std::{ops::Range, path::PathBuf};

use filetime::FileTime;

pub mod file;

pub mod extension;

pub mod entry;

mod access;

pub mod decode;

pub mod verify;

pub mod write;

/// All known versions of a git index file.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Version {
    /// Supports entries and various extensions.
    V2 = 2,
    /// Adds support for additional flags for each entry, called extended entries.
    V3 = 3,
    /// Supports deltified entry paths.
    V4 = 4,
}

/// An entry in the index, identifying a non-tree item on disk.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entry {
    pub stat: entry::Stat,
    pub id: git_hash::ObjectId,
    pub flags: entry::Flags,
    pub mode: entry::Mode,
    path: Range<usize>,
}

/// An index file whose state was read from a file on disk.
pub struct File {
    pub state: State,
    pub path: PathBuf,
    /// The checksum of all bytes prior to the checksum itself.
    pub checksum: git_hash::ObjectId,
}

/// The type to use and store paths to all entries.
pub type PathStorage = Vec<u8>;
/// The type to use and store paths to all entries, as reference
pub type PathStorageRef = [u8];

/// An in-memory cache of a fully parsed git index file.
///
/// As opposed to a snapshot, it's meant to be altered and eventually be written back to disk or converted into a tree.
/// We treat index and its state synonymous.
#[derive(Clone)]
pub struct State {
    /// The time at which the state was created, indicating its freshness compared to other files on disk.
    ///
    /// Note that on platforms that only have a precisions of a second for this time, we will treat all entries with the
    /// same timestamp as this as potentially changed, checking more thoroughly if a change actually happened.
    #[allow(dead_code)]
    timestamp: FileTime,
    version: Version,
    entries: Vec<Entry>,
    /// A memory area keeping all index paths, in full length, independently of the index version.
    path_backing: PathStorage,
    /// True if one entry in the index has a special marker mode
    #[allow(dead_code)]
    is_sparse: bool,

    // Extensions
    tree: Option<extension::Tree>,
    link: Option<extension::Link>,
    resolve_undo: Option<extension::resolve_undo::Paths>,
    untracked: Option<extension::UntrackedCache>,
    fs_monitor: Option<extension::FsMonitor>,
}

pub(crate) mod util {
    use std::convert::TryInto;

    #[inline]
    pub fn var_int(data: &[u8]) -> Option<(u64, &[u8])> {
        let (num, consumed) = git_features::decode::leb64_from_read(data).ok()?;
        let data = &data[consumed..];
        (num, data).into()
    }

    #[inline]
    pub fn read_u32(data: &[u8]) -> Option<(u32, &[u8])> {
        split_at_pos(data, 4).map(|(num, data)| (u32::from_be_bytes(num.try_into().unwrap()), data))
    }

    #[inline]
    pub fn read_u64(data: &[u8]) -> Option<(u64, &[u8])> {
        split_at_pos(data, 8).map(|(num, data)| (u64::from_be_bytes(num.try_into().unwrap()), data))
    }

    #[inline]
    pub fn from_be_u32(b: &[u8]) -> u32 {
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

    #[inline]
    pub fn split_at_pos(data: &[u8], pos: usize) -> Option<(&[u8], &[u8])> {
        if data.len() < pos {
            return None;
        }
        data.split_at(pos).into()
    }
}

#[test]
fn size_of_entry() {
    assert_eq!(std::mem::size_of::<crate::Entry>(), 80);

    // the reason we have our own time is half the size.
    assert_eq!(std::mem::size_of::<crate::entry::Time>(), 8);
    assert_eq!(std::mem::size_of::<filetime::FileTime>(), 16);
}
