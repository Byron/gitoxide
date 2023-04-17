use bstr::BStr;
use gix_index as index;

use crate::status::{Change, VisitEntry};

/// Convenience implementation of [`VisitEntry`] that collects all non-trivial changes into a `Vec`.
#[derive(Debug, Default)]
pub struct Recorder<'index, T = ()> {
    /// collected changes, index entries without conflicts or changes are excluded.
    pub records: Vec<(&'index BStr, Option<Change<T>>, bool)>,
}

impl<'index, T: Send> VisitEntry<'index> for Recorder<'index, T> {
    type ContentChange = T;

    fn visit_entry(
        &mut self,
        _entry: &'index index::Entry,
        rela_path: &'index BStr,
        status: Option<Change<Self::ContentChange>>,
        conflict: bool,
    ) {
        if conflict || status.is_some() {
            self.records.push((rela_path, status, conflict))
        }
    }
}
