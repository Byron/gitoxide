use crate::bstr::BStr;
use git_object::tree::EntryMode;

use crate::Id;

/// An event emitted when finding differences between two trees.
#[derive(Debug, Clone, Copy)]
pub enum Event<'a, 'old, 'new> {
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
    /// Entries are considered renamed if they are not trees and they, according to some understanding of identity, appeared
    /// as [`Deletion`][Event::Deletion] in case of the previous source of the rename as well as [`Addition`][Event::Addition]
    /// acting as destination all the while [rename tracking][super::Platform::track_renames()] is enabled.
    ///
    /// Note that mode changes may have occurred as well, i.e. changes from executable to non-executable or vice-versa.
    Rename {
        /// The location of the source of the rename operation.
        ///
        /// It may be empty if neither [file names][super::Platform::track_filename()] nor [file paths][super::Platform::track_path()]
        /// are tracked.
        source_location: &'a BStr,
        /// The mode of the entry before the rename.
        source_entry_mode: git_object::tree::EntryMode,
        /// The object id of the entry before the rename.
        ///
        /// Note that this is the same as `id` if we require the [similarity to be 100%][super::Renames::percentage], but may
        /// be different otherwise.
        source_id: Id<'old>,

        /// The mode of the entry after the rename.
        /// It could differ but still be considered a rename as we are concerned only about content.
        entry_mode: git_object::tree::EntryMode,
        /// The object id after the rename.
        id: Id<'new>,
    },
    /// This entry is considered to be a copy of another, according to some understanding of identity, as its source still exists.
    /// If the source wouldn't exist, it would be considered a [rename][Event::Rename].
    ///
    /// This variant may only occur if [rename tracking][super::Platform::track_renames()] is enabled, otherwise copies appear to be
    /// plain [additions][Event::Addition].
    Copy {
        /// The location of the source of the copy operation.
        ///
        /// It may be empty if neither [file names][super::Platform::track_filename()] nor [file paths][super::Platform::track_path()]
        /// are tracked.
        source_location: &'a BStr,
        /// The mode of the entry that is considered the source.
        source_entry_mode: git_object::tree::EntryMode,
        /// The object id of the source of the copy.
        ///
        /// Note that this is the same as `id` if we require the [similarity to be 100%][super::Renames::percentage], but may
        /// be different otherwise.
        source_id: Id<'old>,

        /// The mode of the entry after the copy, or the destination of it.
        /// It could differ but still be considered a copy as we are concerned only about content.
        entry_mode: git_object::tree::EntryMode,
        /// The object id after the copy, or the destination of it.
        id: Id<'new>,
    },
}

impl<'a, 'old, 'new> Event<'a, 'old, 'new> {
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

    /// Return the current mode of this instance.
    pub fn entry_mode(&self) -> git_object::tree::EntryMode {
        match self {
            Event::Addition { entry_mode, .. }
            | Event::Deletion { entry_mode, .. }
            | Event::Modification { entry_mode, .. }
            | Event::Rename { entry_mode, .. } => *entry_mode,
            Event::Copy { entry_mode, .. } => *entry_mode,
        }
    }
}
