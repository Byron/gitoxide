//!

/// Internal
impl crate::Repository {
    #[inline]
    pub(crate) fn free_buf(&self) -> Vec<u8> {
        self.bufs.borrow_mut().pop().unwrap_or_default()
    }

    /// This method is commonly called from the destructor of objects that previously claimed an entry
    /// in the free-list with `free_buf()`.
    /// They are welcome to take out the data themselves, for instance when the object is detached, to avoid
    /// it to be reclaimed.
    #[inline]
    pub(crate) fn reuse_buffer(&self, data: &mut Vec<u8>) {
        if data.capacity() > 0 {
            self.bufs.borrow_mut().push(std::mem::take(data));
        }
    }
}

/// Everything else
impl crate::Repository {
    // TODO: actual implementation
    /// Return the committer as configured by this repository, which is determined by…
    ///
    /// * …the git configuration…
    /// * …the GIT_(AUTHOR|COMMITTER)_(NAME|EMAIL|DATE) environment variables…
    ///
    /// …and in that order.
    pub fn committer(&self) -> git_actor::Signature {
        // TODO: actually do the work, probably that should be cached and be refreshable
        git_actor::Signature::empty()
    }

    /// The kind of object hash the repository is configured to use.
    pub fn object_hash(&self) -> git_hash::Kind {
        self.config.object_hash
    }
}

mod worktree {
    use crate::{worktree, Worktree};

    impl crate::Repository {
        /// Return a platform for interacting with worktrees
        pub fn worktree(&self) -> worktree::Platform<'_> {
            worktree::Platform { parent: self }
        }
    }

    impl<'repo> worktree::Platform<'repo> {
        /// Return the currently set worktree if there is one.
        ///
        /// Note that there would be `None` if this repository is `bare` and the parent [`Repository`] was instantiated without
        /// registered worktree in the current working dir.
        pub fn current(&self) -> Option<Worktree<'repo>> {
            self.parent.work_dir().map(|path| Worktree {
                parent: self.parent,
                path,
            })
        }
    }
}

/// Various permissions for parts of git repositories.
pub mod permissions;

mod init;

mod location;

mod snapshots;

mod state;

mod impls;

mod cache;

mod reference;

mod object;

mod thread_safe;
