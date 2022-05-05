use crate::{worktree, Worktree};

impl crate::Repository {
    /// Return true if this repository is bare, and has no main work tree.
    ///
    /// This is not to be confused with the [`current()`][worktree::Platform::current()] worktree, which may exists if this instance
    /// was opened in a worktree that was created separately.
    pub fn is_bare(&self) -> bool {
        self.config.is_bare
    }

    /// Return a platform for interacting with worktrees
    pub fn worktree(&self) -> worktree::Platform<'_> {
        worktree::Platform { parent: self }
    }
}

impl<'repo> worktree::Platform<'repo> {
    /// Return the repository owning the main worktree, if it is not a bare repository.
    pub fn main(&self) -> Option<crate::Repository> {
        todo!()
    }

    /// Return the currently set worktree if there is one.
    ///
    /// Note that there would be `None` if this repository is `bare` and the parent [`Repository`][crate::Repository] was instantiated without
    /// registered worktree in the current working dir.
    pub fn current(&self) -> Option<Worktree<'repo>> {
        self.parent.work_dir().map(|path| Worktree {
            parent: self.parent,
            path,
        })
    }

    /// Iterate all _linked_ worktrees only as a lightweight proxy which needs additional processing to become usable,
    /// but provide a first glimpse a typical worktree information.
    pub fn iter(&self) -> ! {
        todo!()
    }

    /// Iterate all _linked_ worktrees and resolve them, ignoring all prunable ones, into repositories
    /// whose [`current()`][worktree::Platform::current()] is the worktree currently being iterated.
    ///
    /// Note that for convenience all io errors are squelched so if there is a chance for IO errors during
    /// traversal of an owned directory, better use `iter()` directly.
    pub fn iter_repos(&self) -> ! {
        todo!()
    }
}
