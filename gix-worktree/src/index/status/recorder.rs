use bstr::BStr;

use gix_index as index;

use crate::index::status::{Change, Collector};

/// Convenience [`Collecotr`] implementation that collects all non-trivial
/// changes into a `Vec`
#[derive(Debug, Default)]
pub struct Recorder<'index, T = ()> {
    /// collected changes, index entries without conflicts or changes are excluded
    pub records: Vec<(&'index BStr, Option<Change<T>>, bool)>,
}

impl<'index, T: Send> Collector<'index> for Recorder<'index, T> {
    type ContentChange = T;

    fn visit_entry(
        &mut self,
        _entry: &'index index::Entry,
        path: &'index BStr,
        status: Option<Change<Self::ContentChange>>,
        conflict: bool,
    ) {
        if conflict || status.is_some() {
            self.records.push((path, status, conflict))
        }
    }
}
