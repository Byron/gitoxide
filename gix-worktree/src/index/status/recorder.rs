use bstr::BStr;

use gix_index as index;

use crate::index::status::{Collector, Status};

///
#[derive(Debug, Default)]
pub struct Recorder<'index, T = ()> {
    /// collected records, unchanged fields are excluded
    pub records: Vec<(&'index BStr, Status<T>, bool)>,
}

impl<'index, T: Send> Collector<'index> for Recorder<'index, T> {
    type Diff = T;

    fn visit_entry(
        &mut self,
        _entry: &'index index::Entry,
        path: &'index BStr,
        status: Status<Self::Diff>,
        conflict: bool,
    ) {
        if !matches!(status, Status::Unchanged) {
            self.records.push((path, status, conflict))
        }
    }
}
