//!

use git_ref::file::ReferenceExt;

use crate::easy::Reference;

impl<'repo> Reference<'repo> {
    /// Return a platform for obtaining iterators over reference logs.
    pub fn log_iter(&self) -> git_ref::file::log::iter::Platform<'_, '_> {
        self.inner.log_iter(&self.handle.refs)
    }
}
