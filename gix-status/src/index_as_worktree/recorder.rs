use bstr::BStr;
use gix_index as index;

use crate::index_as_worktree::{EntryStatus, VisitEntry};

/// A record of a change.
///
/// It's created either if there is a conflict or a change, or both.
#[derive(Debug)]
pub struct Record<'index, T, U> {
    /// The index entry that is changed.
    pub entry: &'index index::Entry,
    /// The index of the `entry` relative to all entries in the input index.
    pub entry_index: usize,
    /// The path to the entry.
    pub relative_path: &'index BStr,
    /// The status information itself.
    pub status: EntryStatus<T, U>,
}

/// Convenience implementation of [`VisitEntry`] that collects all non-trivial changes into a `Vec`.
#[derive(Debug, Default)]
pub struct Recorder<'index, T = (), U = ()> {
    /// collected changes, index entries without conflicts or changes are excluded.
    pub records: Vec<Record<'index, T, U>>,
}

impl<'index, T: Send, U: Send> VisitEntry<'index> for Recorder<'index, T, U> {
    type ContentChange = T;
    type SubmoduleStatus = U;

    fn visit_entry(
        &mut self,
        _entries: &'index [index::Entry],
        entry: &'index index::Entry,
        entry_index: usize,
        relative_path: &'index BStr,
        status: EntryStatus<Self::ContentChange, Self::SubmoduleStatus>,
    ) {
        self.records.push(Record {
            entry,
            entry_index,
            relative_path,
            status,
        })
    }
}
