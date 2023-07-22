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
/// Used in [`mutable::Entry`][crate::tree::Entry] and [`EntryRef`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd, Hash)]
#[repr(u16)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    /// Return true if the entry is any kind of blob.
    pub fn is_blob(&self) -> bool {
        matches!(self, EntryMode::Blob | EntryMode::BlobExecutable)
    }

    /// Return true if the entry is any kind of blob or symlink.
    pub fn is_blob_or_symlink(&self) -> bool {
        matches!(self, EntryMode::Blob | EntryMode::BlobExecutable | EntryMode::Link)
    }

    /// Represent the mode as descriptive string.
    pub fn as_str(&self) -> &'static str {
        use EntryMode::*;
        match self {
            Tree => "tree",
            Blob => "blob",
            BlobExecutable => "exe",
            Link => "link",
            Commit => "commit",
        }
    }
}

/// An element of a [`TreeRef`][crate::TreeRef::entries].
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntryRef<'a> {
    /// The kind of object to which `oid` is pointing.
    pub mode: tree::EntryMode,
    /// The name of the file in the parent tree.
    pub filename: &'a BStr,
    /// The id of the object representing the entry.
    // TODO: figure out how these should be called. id or oid? It's inconsistent around the codebase.
    //       Answer: make it 'id', as in `git2`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub oid: &'a gix_hash::oid,
}

impl<'a> PartialOrd for EntryRef<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for EntryRef<'a> {
    fn cmp(&self, b: &Self) -> Ordering {
        let a = self;
        let common = a.filename.len().min(b.filename.len());
        a.filename[..common].cmp(&b.filename[..common]).then_with(|| {
            let a = a.filename.get(common).or_else(|| a.mode.is_tree().then_some(&b'/'));
            let b = b.filename.get(common).or_else(|| b.mode.is_tree().then_some(&b'/'));
            a.cmp(&b)
        })
    }
}

/// An entry in a [`Tree`][crate::Tree], similar to an entry in a directory.
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The kind of object to which `oid` is pointing to.
    pub mode: EntryMode,
    /// The name of the file in the parent tree.
    pub filename: BString,
    /// The id of the object representing the entry.
    pub oid: gix_hash::ObjectId,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, b: &Self) -> Ordering {
        let a = self;
        let common = a.filename.len().min(b.filename.len());
        a.filename[..common].cmp(&b.filename[..common]).then_with(|| {
            let a = a.filename.get(common).or_else(|| a.mode.is_tree().then_some(&b'/'));
            let b = b.filename.get(common).or_else(|| b.mode.is_tree().then_some(&b'/'));
            a.cmp(&b)
        })
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
