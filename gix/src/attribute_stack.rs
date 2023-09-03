use crate::bstr::BStr;
use crate::types::AttributeStack;
use crate::Repository;
use gix_odb::FindExt;
use std::ops::{Deref, DerefMut};

/// Lifecycle
impl<'repo> AttributeStack<'repo> {
    /// Create a new instance from a `repo` and the underlying pre-configured `stack`.
    ///
    /// Note that this type is typically created by [`Repository::attributes()`] or [`Repository::attributes_only()`].
    pub fn new(stack: gix_worktree::Stack, repo: &'repo Repository) -> Self {
        AttributeStack { repo, inner: stack }
    }

    /// Detach the repository and return the underlying plumbing datatype.
    pub fn detach(self) -> gix_worktree::Stack {
        self.inner
    }
}

impl Deref for AttributeStack<'_> {
    type Target = gix_worktree::Stack;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AttributeStack<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Platform retrieval
impl<'repo> AttributeStack<'repo> {
    /// Append the `relative` path to the root directory of the cache and efficiently create leading directories, while assuring that no
    /// symlinks are in that path.
    /// Unless `is_dir` is known with `Some(…)`, then `relative` points to a directory itself in which case the entire resulting
    /// path is created as directory. If it's not known it is assumed to be a file.
    ///
    /// Provide access to cached information for that `relative` path via the returned platform.
    pub fn at_path(
        &mut self,
        relative: impl AsRef<std::path::Path>,
        is_dir: Option<bool>,
    ) -> std::io::Result<gix_worktree::stack::Platform<'_>> {
        self.inner
            .at_path(relative, is_dir, |id, buf| self.repo.objects.find_blob(id, buf))
    }

    /// Obtain a platform for lookups from a repo-`relative` path, typically obtained from an index entry. `is_dir` should reflect
    /// whether it's a directory or not, or left at `None` if unknown.
    ///
    /// If `relative` ends with `/` and `is_dir` is `None`, it is automatically assumed to be a directory.
    ///
    /// ### Panics
    ///
    /// - on illformed UTF8 in `relative`
    pub fn at_entry<'r>(
        &mut self,
        relative: impl Into<&'r BStr>,
        is_dir: Option<bool>,
    ) -> std::io::Result<gix_worktree::stack::Platform<'_>> {
        self.inner
            .at_entry(relative, is_dir, |id, buf| self.repo.objects.find_blob(id, buf))
    }
}
