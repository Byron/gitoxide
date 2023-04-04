use bstr::BStr;

use gix_index as index;

use crate::fs;
use crate::index::status::visit;

/// A [Visit][visit::Visit] implementation to record every observed change and keep track of the changed paths.
#[derive(Debug)]
pub struct Recorder<'a, 'index> {
    index: &'index index::State,
    buf: Vec<u8>,
    capabilities: &'a fs::Capabilities,
    ///
    pub records: Vec<(&'index BStr, visit::worktree::Status, bool)>,
}

impl<'a, 'index> Recorder<'a, 'index> {
    ///
    pub fn new(capabilities: &'a fs::Capabilities, index: &'index index::State) -> Self {
        Recorder {
            index,
            buf: Vec::with_capacity(8 * 1024),
            capabilities,
            records: Vec::new(),
        }
    }
}

impl<'a, 'index> visit::worktree::Visit<'index> for Recorder<'a, 'index> {
    fn visit_entry(
        &mut self,
        entry: &'index index::Entry,
        status: Result<visit::worktree::Status, visit::worktree::Error>,
        path: Result<&std::path::Path, &BStr>,
        conflict: bool,
    ) {
        // we treat any errors as a data modiciation to be conservative
        let status = if let Ok(path) = path {
            if let Ok(mut status) = status {
                let _ = status.compare_data(path, entry, &mut self.buf, self.capabilities);
                status
            } else {
                visit::worktree::Status::Modified(visit::Modification {
                    mode_change: None,
                    stat_changed: true,
                    data_changed: true,
                })
            }
        } else {
            visit::worktree::Status::Modified(visit::Modification {
                mode_change: None,
                stat_changed: true,
                data_changed: true,
            })
        };
        let path = entry.path(self.index);
        if status != visit::worktree::Status::Unchanged {
            self.records.push((path, status, conflict))
        }
    }
}
