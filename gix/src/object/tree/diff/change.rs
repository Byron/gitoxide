use crate::{bstr::BStr, Id};

/// Information about the diff performed to detect similarity of a [Rewrite][Event::Rewrite].
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct DiffLineStats {
    /// The amount of lines to remove from the source to get to the destination.
    pub removals: u32,
    /// The amount of lines to add to the source to get to the destination.
    pub insertions: u32,
    /// The amount of lines of the previous state, in the source.
    pub before: u32,
    /// The amount of lines of the new state, in the destination.
    pub after: u32,
}

/// An event emitted when finding differences between two trees.
#[derive(Debug, Clone, Copy)]
pub enum Event<'a, 'old, 'new> {
    /// An entry was added, like the addition of a file or directory.
    Addition {
        /// The mode of the added entry.
        entry_mode: gix_object::tree::EntryMode,
        /// The object id of the added entry.
        id: Id<'new>,
    },
    /// An entry was deleted, like the deletion of a file or directory.
    Deletion {
        /// The mode of the deleted entry.
        entry_mode: gix_object::tree::EntryMode,
        /// The object id of the deleted entry.
        id: Id<'old>,
    },
    /// An entry was modified, e.g. changing the contents of a file adjusts its object id and turning
    /// a file into a symbolic link adjusts its mode.
    Modification {
        /// The mode of the entry before the modification.
        previous_entry_mode: gix_object::tree::EntryMode,
        /// The object id of the entry before the modification.
        previous_id: Id<'old>,

        /// The mode of the entry after the modification.
        entry_mode: gix_object::tree::EntryMode,
        /// The object id after the modification.
        id: Id<'new>,
    },
    /// Entries are considered rewritten if they are not trees and they, according to some understanding of identity, were renamed
    /// or copied.
    /// In case of renames, this means they originally appeared as [`Deletion`][Event::Deletion] signalling their source as well as an
    /// [`Addition`][Event::Addition] acting as destination.
    ///
    /// In case of copies, the `copy` flag is true and typically represents a perfect copy of a source was made.
    ///
    /// This variant can only be encountered if [rewrite tracking][super::Platform::track_rewrites()] is enabled.
    ///
    /// Note that mode changes may have occurred as well, i.e. changes from executable to non-executable or vice-versa.
    Rewrite {
        /// The location of the source of the rename operation.
        ///
        /// It may be empty if neither [file names][super::Platform::track_filename()] nor [file paths][super::Platform::track_path()]
        /// are tracked.
        source_location: &'a BStr,
        /// The mode of the entry before the rename.
        source_entry_mode: gix_object::tree::EntryMode,
        /// The object id of the entry before the rename.
        ///
        /// Note that this is the same as `id` if we require the [similarity to be 100%][super::Rewrites::percentage], but may
        /// be different otherwise.
        source_id: Id<'old>,
        /// Information about the diff we performed to detect similarity and match the `source_id` with the current state at `id`.
        /// It's `None` if `source_id` is equal to `id`, as identity made an actual diff computation unnecessary.
        diff: Option<DiffLineStats>,
        /// The mode of the entry after the rename.
        /// It could differ but still be considered a rename as we are concerned only about content.
        entry_mode: gix_object::tree::EntryMode,
        /// The object id after the rename.
        id: Id<'new>,
        /// If true, this rewrite is created by copy, and `source_id` is pointing to its source. Otherwise it's a rename, and `source_id`
        /// points to a deleted object, as renames are tracked as deletions and additions of the same or similar content.
        copy: bool,
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
                previous_entry_mode,
                previous_id,
                entry_mode,
                id,
            } if entry_mode.is_blob() && previous_entry_mode.is_blob() => {
                Some(crate::object::blob::diff::Platform::from_ids(previous_id, id))
            }
            _ => None,
        }
    }

    /// Return the current mode of this instance.
    pub fn entry_mode(&self) -> gix_object::tree::EntryMode {
        match self {
            Event::Addition { entry_mode, .. }
            | Event::Deletion { entry_mode, .. }
            | Event::Modification { entry_mode, .. }
            | Event::Rewrite { entry_mode, .. } => *entry_mode,
        }
    }
}
