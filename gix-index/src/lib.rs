//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(unsafe_code, missing_docs, rust_2018_idioms)]

use std::{ops::Range, path::PathBuf};

use filetime::FileTime;
pub use gix_hash as hash;

///
pub mod file;

///
pub mod extension;

///
pub mod entry;

mod access;

mod init;

///
pub mod decode;

///
pub mod verify;

///
pub mod write;

/// All known versions of a git index file.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// The filesystem stat information for the file on disk.
    pub stat: entry::Stat,
    /// The object id for this entry's ODB representation (assuming it's up-to-date with it).
    pub id: gix_hash::ObjectId,
    /// Additional flags for use in algorithms and for efficiently storing stage information.
    pub flags: entry::Flags,
    /// The kind of item this entry represents - it's not all blobs in the index anymore.
    pub mode: entry::Mode,
    /// The range to lookup in the path backing to obtain the entry path relative to the repository.
    /// This costs additional memory but is probably worth it given that paths can stay in one big allocation.
    path: Range<usize>,
}

/// An index file whose state was read from a file on disk.
#[derive(Clone)]
pub struct File {
    /// The state containing the actual index data.
    pub(crate) state: State,
    /// The path from which the index was read or to which it is supposed to be written.
    pub(crate) path: PathBuf,
    /// The checksum of all bytes prior to the checksum itself.
    pub(crate) checksum: Option<gix_hash::ObjectId>,
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
    /// The kind of object hash used when storing the underlying file.
    ///
    /// Empty states for example won't have a single object id, so deduction of the hash used isn't always possible.
    object_hash: gix_hash::Kind,
    /// The time at which the state was created, indicating its freshness compared to other files on disk.
    ///
    /// Note that on platforms that only have a precisions of a second for this time, we will treat all entries with the
    /// same timestamp as this as potentially changed, checking more thoroughly if a change actually happened.
    timestamp: FileTime,
    version: Version,
    entries: Vec<Entry>,
    /// A memory area keeping all index paths, in full length, independently of the index version.
    ///
    /// Ranges into this storage are referred to by parts of `entries`.
    path_backing: PathStorage,
    /// True if one entry in the index has a special marker mode
    is_sparse: bool,

    // Extensions
    tree: Option<extension::Tree>,
    link: Option<extension::Link>,
    resolve_undo: Option<extension::resolve_undo::Paths>,
    untracked: Option<extension::UntrackedCache>,
    fs_monitor: Option<extension::FsMonitor>,
}

mod impls {
    use std::fmt::{Debug, Formatter};

    use crate::State;

    impl Debug for State {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for entry in &self.entries {
                writeln!(
                    f,
                    "{} {}{:?} {} {}",
                    match entry.flags.stage() {
                        0 => "       ",
                        1 => "BASE   ",
                        2 => "OURS   ",
                        3 => "THEIRS ",
                        _ => "UNKNOWN",
                    },
                    if entry.flags.is_empty() {
                        "".to_string()
                    } else {
                        format!("{:?} ", entry.flags)
                    },
                    entry.mode,
                    entry.id,
                    entry.path(self)
                )?;
            }
            Ok(())
        }
    }
}

pub(crate) mod util {
    use std::convert::TryInto;

    #[inline]
    pub fn var_int(data: &[u8]) -> Option<(u64, &[u8])> {
        let (num, consumed) = gix_features::decode::leb64_from_read(data).ok()?;
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
    assert_eq!(std::mem::size_of::<crate::entry::stat::Time>(), 8);
    assert_eq!(std::mem::size_of::<filetime::FileTime>(), 16);
}
