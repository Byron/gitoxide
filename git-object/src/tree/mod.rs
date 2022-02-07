use std::cmp::Ordering;

use crate::{
    bstr::{BStr, BString},
    tree,
};

mod ref_iter;
///
pub mod write;

/// The mode of items storable in a tree, similar to the file mode on a unix file system.
///
/// Used in [mutable::Entry][crate::tree::Entry] and [EntryRef].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd, Hash)]
#[repr(u16)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum EntryMode {
    /// A tree, or directory
    Tree = 0o040000u16,
    /// A file that is not executable
    Blob = 0o100644,
    /// A file that is executable
    BlobExecutable = 0o100755,
    /// A symbolic link
    Link = 0o120000,
    /// A commit of a git submodule
    Commit = 0o160000,
}

impl EntryMode {
    /// Return true if this entry mode represents a Tree/directory
    pub fn is_tree(&self) -> bool {
        *self == EntryMode::Tree
    }

    /// Return true if this entry mode represents anything BUT Tree/directory
    pub fn is_no_tree(&self) -> bool {
        *self != EntryMode::Tree
    }
}

/// An element of a [`TreeRef`][crate::TreeRef::entries].
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct EntryRef<'a> {
    /// The kind of object to which `oid` is pointing.
    pub mode: tree::EntryMode,
    /// The name of the file in the parent tree.
    pub filename: &'a BStr,
    /// The id of the object representing the entry.
    // TODO: figure out how these should be called. id or oid? It's inconsistent around the codebase.
    // Answer: make it 'id', as in `git2`
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub oid: &'a git_hash::oid,
}

impl<'a> PartialOrd for EntryRef<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for EntryRef<'a> {
    /// Entries compare by the common portion of the filename. This is critical for proper functioning of algorithms working on trees.
    fn cmp(&self, other: &Self) -> Ordering {
        let len = self.filename.len().min(other.filename.len());
        self.filename[..len].cmp(&other.filename[..len])
    }
}

/// An entry in a [`Tree`][crate::Tree], similar to an entry in a directory.
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The kind of object to which `oid` is pointing to.
    pub mode: EntryMode,
    /// The name of the file in the parent tree.
    pub filename: BString,
    /// The id of the object representing the entry.
    pub oid: git_hash::ObjectId,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    /// Entries compare by the common portion of the filename. This is critical for proper functioning of algorithms working on trees.
    fn cmp(&self, other: &Self) -> Ordering {
        let common_len = self.filename.len().min(other.filename.len());
        self.filename[..common_len]
            .cmp(&other.filename[..common_len])
            .then_with(|| self.filename.len().cmp(&other.filename.len()))
    }
}

/// Serialization
impl EntryMode {
    /// Return the representation as used in the git internal format.
    pub fn as_bytes(&self) -> &'static [u8] {
        use EntryMode::*;
        match self {
            Tree => b"40000",
            Blob => b"100644",
            BlobExecutable => b"100755",
            Link => b"120000",
            Commit => b"160000",
        }
    }
}
