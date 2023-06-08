use crate::{worktree, Worktree};

/// Interact with individual worktrees and their information.
impl crate::Repository {
    /// Return a list of all _linked_ worktrees sorted by private git dir path as a lightweight proxy.
    ///
    /// Note that these need additional processing to become usable, but provide a first glimpse a typical worktree information.
    pub fn worktrees(&self) -> std::io::Result<Vec<worktree::Proxy<'_>>> {
        let mut res = Vec::new();
        let iter = match std::fs::read_dir(self.common_dir().join("worktrees")) {
            Ok(iter) => iter,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(res),
            Err(err) => return Err(err),
        };
        for entry in iter {
            let entry = entry?;
            let worktree_git_dir = entry.path();
            if worktree_git_dir.join("gitdir").is_file() {
                res.push(worktree::Proxy {
                    parent: self,
                    git_dir: worktree_git_dir,
                })
            }
        }
        res.sort_by(|a, b| a.git_dir.cmp(&b.git_dir));
        Ok(res)
    }
    /// Return the repository owning the main worktree, typically from a linked worktree.
    ///
    /// Note that it might be the one that is currently open if this repository doesn't point to a linked worktree.
    /// Also note that the main repo might be bare.
    #[allow(clippy::result_large_err)]
    pub fn main_repo(&self) -> Result<crate::Repository, crate::open::Error> {
        crate::ThreadSafeRepository::open_opts(self.common_dir(), self.options.clone()).map(Into::into)
    }

    /// Return the currently set worktree if there is one, acting as platform providing a validated worktree base path.
    ///
    /// Note that there would be `None` if this repository is `bare` and the parent [`Repository`][crate::Repository] was instantiated without
    /// registered worktree in the current working dir.
    pub fn worktree(&self) -> Option<Worktree<'_>> {
        self.work_dir().map(|path| Worktree { parent: self, path })
    }

    /// Return true if this repository is bare, and has no main work tree.
    ///
    /// This is not to be confused with the [`worktree()`][crate::Repository::worktree()] worktree, which may exists if this instance
    /// was opened in a worktree that was created separately.
    pub fn is_bare(&self) -> bool {
        self.config.is_bare && self.work_dir().is_none()
    }
}
