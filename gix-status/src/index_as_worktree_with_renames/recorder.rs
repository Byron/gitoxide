use crate::index_as_worktree_with_renames::{Entry, VisitEntry};

/// Convenience implementation of [`VisitEntry`] that collects all changes into a `Vec`.
#[derive(Debug, Default)]
pub struct Recorder<'index, T = (), U = ()> {
    /// The collected changes.
    pub records: Vec<Entry<'index, T, U>>,
}

impl<'index, T: Send, U: Send> VisitEntry<'index> for Recorder<'index, T, U> {
    type ContentChange = T;
    type SubmoduleStatus = U;

    fn visit_entry(&mut self, entry: Entry<'index, Self::ContentChange, Self::SubmoduleStatus>) {
        self.records.push(entry)
    }
}
