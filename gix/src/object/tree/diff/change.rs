use crate::bstr::BString;
use crate::ext::ObjectIdExt;
use crate::object::tree::diff::ChangeDetached;
use crate::{bstr::BStr, diff::blob::DiffLineStats, Id, Repository};

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
        /// If true, this rewrite is created by copy, and `source_id` is pointing to its source. Otherwise, it's a rename, and `source_id`
        /// points to a deleted object, as renames are tracked as deletions and additions of the same or similar content.
        copy: bool,
    },
}

/// An event emitted when finding differences between two trees.
#[derive(Debug, Clone)]
pub enum EventDetached {
    /// An entry was added, like the addition of a file or directory.
    Addition {
        /// The mode of the added entry.
        entry_mode: gix_object::tree::EntryMode,
        /// The object id of the added entry.
        id: gix_hash::ObjectId,
    },
    /// An entry was deleted, like the deletion of a file or directory.
    Deletion {
        /// The mode of the deleted entry.
        entry_mode: gix_object::tree::EntryMode,
        /// The object id of the deleted entry.
        id: gix_hash::ObjectId,
    },
    /// An entry was modified, e.g. changing the contents of a file adjusts its object id and turning
    /// a file into a symbolic link adjusts its mode.
    Modification {
        /// The mode of the entry before the modification.
        previous_entry_mode: gix_object::tree::EntryMode,
        /// The object id of the entry before the modification.
        previous_id: gix_hash::ObjectId,

        /// The mode of the entry after the modification.
        entry_mode: gix_object::tree::EntryMode,
        /// The object id after the modification.
        id: gix_hash::ObjectId,
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
        source_location: BString,
        /// The mode of the entry before the rename.
        source_entry_mode: gix_object::tree::EntryMode,
        /// The object id of the entry before the rename.
        ///
        /// Note that this is the same as `id` if we require the [similarity to be 100%][super::Rewrites::percentage], but may
        /// be different otherwise.
        source_id: gix_hash::ObjectId,
        /// Information about the diff we performed to detect similarity and match the `source_id` with the current state at `id`.
        /// It's `None` if `source_id` is equal to `id`, as identity made an actual diff computation unnecessary.
        diff: Option<DiffLineStats>,
        /// The mode of the entry after the rename.
        /// It could differ but still be considered a rename as we are concerned only about content.
        entry_mode: gix_object::tree::EntryMode,
        /// The object id after the rename.
        id: gix_hash::ObjectId,
        /// If true, this rewrite is created by copy, and `source_id` is pointing to its source. Otherwise, it's a rename, and `source_id`
        /// points to a deleted object, as renames are tracked as deletions and additions of the same or similar content.
        copy: bool,
    },
}

/// Lifecycle
impl super::Change<'_, '_, '_> {
    /// Detach the repository instance to obtain a fully-owned version
    pub fn detach(self) -> ChangeDetached {
        ChangeDetached {
            location: self.location.to_owned(),
            event: self.event.detach(),
        }
    }
}

/// Lifecycle
impl ChangeDetached {
    /// Return an attached version of this instance that uses `old_repo` for previous values and `new_repo` for current values.
    pub fn attach<'old, 'new>(
        &self,
        old_repo: &'old Repository,
        new_repo: &'new Repository,
    ) -> super::Change<'_, 'old, 'new> {
        super::Change {
            location: self.location.as_ref(),
            event: self.event.attach(old_repo, new_repo),
        }
    }
}

impl<'a, 'old, 'new> super::Change<'a, 'old, 'new> {
    /// Produce a platform for performing a line-diff no matter whether the underlying [Event] is an addition, modification,
    /// deletion or rewrite.
    /// Use `resource_cache` to store the diffable data and possibly reuse previously stored data, usually obtained with
    /// [crate::Repository::diff_resource_cache()].
    /// Afterward the platform, which holds on to `resource_cache`, can be used to perform ready-made operations on the
    /// pre-set resources.
    ///
    /// ### Warning about Memory Consumption
    ///
    /// `resource_cache` only grows, so one should call [`gix_diff::blob::Platform::clear_resource_cache`] occasionally.
    pub fn diff<'b>(
        &self,
        resource_cache: &'b mut gix_diff::blob::Platform,
    ) -> Result<crate::object::blob::diff::Platform<'b>, crate::object::blob::diff::init::Error> {
        crate::object::blob::diff::Platform::from_tree_change(self, resource_cache)
    }
}

/// Lifecycle
impl Event<'_, '_, '_> {
    /// Detach the repository instance to obtain a fully-owned version
    pub fn detach(self) -> EventDetached {
        match self {
            Event::Addition { entry_mode, id } => EventDetached::Addition {
                entry_mode,
                id: id.detach(),
            },
            Event::Deletion { entry_mode, id } => EventDetached::Deletion {
                entry_mode,
                id: id.detach(),
            },
            Event::Modification {
                previous_entry_mode,
                previous_id,
                entry_mode,
                id,
            } => EventDetached::Modification {
                previous_entry_mode,
                previous_id: previous_id.detach(),
                entry_mode,
                id: id.detach(),
            },
            Event::Rewrite {
                source_location,
                source_entry_mode,
                source_id,
                diff,
                entry_mode,
                id,
                copy,
            } => EventDetached::Rewrite {
                source_location: source_location.to_owned(),
                source_entry_mode,
                source_id: source_id.detach(),
                diff,
                entry_mode,
                id: id.detach(),
                copy,
            },
        }
    }
}

impl EventDetached {
    /// Return an attached version of this instance that uses `old_repo` for previous values and `new_repo` for current values.
    pub fn attach<'old, 'new>(&self, old_repo: &'old Repository, new_repo: &'new Repository) -> Event<'_, 'old, 'new> {
        match self {
            EventDetached::Addition { entry_mode, id } => Event::Addition {
                entry_mode: *entry_mode,
                id: id.attach(new_repo),
            },
            EventDetached::Deletion { entry_mode, id } => Event::Deletion {
                entry_mode: *entry_mode,
                id: id.attach(old_repo),
            },
            EventDetached::Modification {
                previous_entry_mode,
                previous_id,
                entry_mode,
                id,
            } => Event::Modification {
                previous_entry_mode: *previous_entry_mode,
                previous_id: previous_id.attach(old_repo),
                entry_mode: *entry_mode,
                id: id.attach(new_repo),
            },
            EventDetached::Rewrite {
                source_location,
                source_entry_mode,
                source_id,
                diff,
                entry_mode,
                id,
                copy,
            } => Event::Rewrite {
                source_location: source_location.as_ref(),
                source_entry_mode: *source_entry_mode,
                source_id: source_id.attach(old_repo),
                diff: *diff,
                entry_mode: *entry_mode,
                id: id.attach(new_repo),
                copy: *copy,
            },
        }
    }
}

impl<'a, 'old, 'new> Event<'a, 'old, 'new> {
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
