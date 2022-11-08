use git_object::tree::EntryMode;

use crate::Id;

/// An event emitted when finding differences between two trees.
#[derive(Debug, Clone, Copy)]
pub enum Event<'old, 'new> {
    /// An entry was added, like the addition of a file or directory.
    Addition {
        /// The mode of the added entry.
        entry_mode: git_object::tree::EntryMode,
        /// The object id of the added entry.
        id: Id<'new>,
    },
    /// An entry was deleted, like the deletion of a file or directory.
    Deletion {
        /// The mode of the deleted entry.
        entry_mode: git_object::tree::EntryMode,
        /// The object id of the deleted entry.
        id: Id<'old>,
    },
    /// An entry was modified, e.g. changing the contents of a file adjusts its object id and turning
    /// a file into a symbolic link adjusts its mode.
    Modification {
        /// The mode of the entry before the modification.
        previous_entry_mode: git_object::tree::EntryMode,
        /// The object id of the entry before the modification.
        previous_id: Id<'old>,

        /// The mode of the entry after the modification.
        entry_mode: git_object::tree::EntryMode,
        /// The object id after the modification.
        id: Id<'new>,
    },
}

impl<'old, 'new> Event<'old, 'new> {
    /// Produce a platform for performing a line-diff, or `None` if this is not a [`Modification`][Event::Modification]
    /// or one of the entries to compare is not a blob.
    pub fn diff(
        &self,
    ) -> Option<Result<crate::object::blob::diff::Platform<'old, 'new>, crate::object::blob::diff::init::Error>> {
        match self {
            Event::Modification {
                previous_entry_mode: EntryMode::BlobExecutable | EntryMode::Blob,
                previous_id,
                entry_mode: EntryMode::BlobExecutable | EntryMode::Blob,
                id,
            } => Some(crate::object::blob::diff::Platform::from_ids(previous_id, id)),
            _ => None,
        }
    }
}
