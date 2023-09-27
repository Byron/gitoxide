use bstr::BStr;
use gix_index as index;

use crate::index_as_worktree::{Change, VisitEntry};

/// A record of a change.
///
/// It's created either if there is a conflict or a change, or both.
#[derive(Debug)]
pub struct Record<'index, T, U> {
    /// The index entry that is changed.
    pub entry: &'index index::Entry,
    /// The path to the entry.
    pub relative_path: &'index BStr,
    /// The change itself, or `None` if there is only a conflict.
    pub change: Option<Change<T, U>>,
    /// information about the conflict itself
    pub conflict: bool,
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
        entry: &'index index::Entry,
        rela_path: &'index BStr,
        change: Option<Change<Self::ContentChange, Self::SubmoduleStatus>>,
        conflict: bool,
    ) {
        if conflict || change.is_some() {
            self.records.push(Record {
                entry,
                relative_path: rela_path,
                change,
                conflict,
            })
        }
    }
}
