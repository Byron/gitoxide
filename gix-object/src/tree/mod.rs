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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntryMode(u16);

/// A discretized version of ideal and valid values for entry modes.
///
/// Note that even though it can represent every valid [mode](EntryMode), it might
/// loose information due to that as well.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd, Hash)]
#[repr(u16)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EntryKind {
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

impl From<EntryKind> for EntryMode {
    fn from(value: EntryKind) -> Self {
        EntryMode(value as u16)
    }
}

impl From<EntryMode> for EntryKind {
    fn from(value: EntryMode) -> Self {
        value.kind()
    }
}

/// Serialization
impl EntryKind {
    /// Return the representation as used in the git internal format.
    pub fn as_octal_str(&self) -> &'static BStr {
        use EntryKind::*;
        let bytes: &[u8] = match self {
            Tree => b"40000",
            Blob => b"100644",
            BlobExecutable => b"100755",
            Link => b"120000",
            Commit => b"160000",
        };
        bytes.into()
    }
}

impl std::ops::Deref for EntryMode {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EntryMode {
    /// Discretize the raw mode into an enum with well-known state while dropping unnecessary details.
    pub const fn kind(&self) -> EntryKind {
        match self.0 {
            0o40000 => EntryKind::Tree,
            0o120000 => EntryKind::Link,
            0o160000 => EntryKind::Commit,
            blob_mode => {
                if blob_mode & 0o000100 == 0o000100 {
                    EntryKind::BlobExecutable
                } else {
                    EntryKind::Blob
                }
            }
        }
    }

    /// Return true if this entry mode represents a Tree/directory
    pub const fn is_tree(&self) -> bool {
        self.0 == EntryKind::Tree as u16
    }

    /// Return true if this entry mode represents the commit of a submodule.
    pub const fn is_commit(&self) -> bool {
        self.0 == EntryKind::Commit as u16
    }

    /// Return true if this entry mode represents a symbolic link
    pub const fn is_link(&self) -> bool {
        self.0 == EntryKind::Link as u16
    }

    /// Return true if this entry mode represents anything BUT Tree/directory
    pub const fn is_no_tree(&self) -> bool {
        self.0 != EntryKind::Tree as u16
    }

    /// Return true if the entry is any kind of blob.
    pub const fn is_blob(&self) -> bool {
        matches!(self.kind(), EntryKind::Blob | EntryKind::BlobExecutable)
    }

    /// Return true if the entry is an executable blob.
    pub const fn is_executable(&self) -> bool {
        matches!(self.kind(), EntryKind::BlobExecutable)
    }

    /// Return true if the entry is any kind of blob or symlink.
    pub const fn is_blob_or_symlink(&self) -> bool {
        matches!(
            self.kind(),
            EntryKind::Blob | EntryKind::BlobExecutable | EntryKind::Link
        )
    }

    /// Represent the mode as descriptive string.
    pub const fn as_str(&self) -> &'static str {
        use EntryKind::*;
        match self.kind() {
            Tree => "tree",
            Blob => "blob",
            BlobExecutable => "exe",
            Link => "link",
            Commit => "commit",
        }
    }

    /// Return the representation as used in the git internal format, which is octal and written
    /// to the `backing` buffer. The respective sub-slice that was written to is returned.
    pub fn as_bytes<'a>(&self, backing: &'a mut [u8; 6]) -> &'a BStr {
        if self.0 == 0 {
            std::slice::from_ref(&b'0')
        } else {
            let mut nb = 0;
            let mut n = self.0;
            while n > 0 {
                let remainder = (n % 8) as u8;
                backing[nb] = b'0' + remainder;
                n /= 8;
                nb += 1;
            }
            let res = &mut backing[..nb];
            res.reverse();
            res
        }
        .into()
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
