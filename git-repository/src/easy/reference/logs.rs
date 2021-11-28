//!

use git_ref::file::ReferenceExt;

use crate::{easy, easy::Reference};

impl<'repo, A> Reference<'repo, A>
where
    A: easy::Access + Sized,
{
    /// Return a platform for obtaining iterators over reference logs.
    pub fn log_iter(&self) -> git_ref::file::log::iter::Platform<'_, '_> {
        self.inner.log_iter(&self.access.state().refs)
    }
}
