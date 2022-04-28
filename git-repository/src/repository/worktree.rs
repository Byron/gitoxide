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
    /// Note that there would be `None` if this repository is `bare` and the parent [`Repository`][crate::Repository] was instantiated without
    /// registered worktree in the current working dir.
    pub fn current(&self) -> Option<Worktree<'repo>> {
        self.parent.work_dir().map(|path| Worktree {
            parent: self.parent,
            path,
        })
    }
}
