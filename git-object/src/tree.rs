/// The mode of items storable in a tree, similar to the file mode on a unix file system.
///
/// Used in [mutable::Entry][crate::mutable::tree::Entry] and [EntryRef].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd, Hash)]
#[repr(u16)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum EntryMode {
    Tree = 0o040000u16,
    Blob = 0o100644,
    BlobExecutable = 0o100755,
    Link = 0o120000,
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

pub use crate::immutable::tree::{EntryRef, RefIter};
