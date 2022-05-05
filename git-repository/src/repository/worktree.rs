use crate::bstr::BString;
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

    /// Open a new copy of the index file and decode it entirely.
    ///
    /// It will use the `index.threads` configuration key to learn how many threads to use.
    /// Note that it may fail if there is no index.
    // TODO: test
    #[cfg(feature = "git-index")]
    pub fn open_index(&self) -> Result<git_index::File, crate::worktree::open_index::Error> {
        use std::convert::{TryFrom, TryInto};
        let thread_limit = self
            .config
            .resolved
            .boolean("index", None, "threads")
            .map(|res| {
                res.map(|value| if value { 0usize } else { 1 }).or_else(|err| {
                    git_config::values::Integer::try_from(err.input.as_ref())
                        .map_err(|err| crate::worktree::open_index::Error::ConfigIndexThreads {
                            value: err.input.clone(),
                            err,
                        })
                        .map(|value| value.to_decimal().and_then(|v| v.try_into().ok()).unwrap_or(1))
                })
            })
            .transpose()?;
        git_index::File::at(
            self.git_dir().join("index"),
            git_index::decode::Options {
                object_hash: self.object_hash(),
                thread_limit,
                min_extension_block_in_bytes_for_threading: 0,
            },
        )
        .map_err(Into::into)
    }

    /// Return true if the worktree cannot be pruned, moved or deleted, which is useful if it is located on an external storage device.
    pub fn is_locked(&self) -> bool {
        todo!()
    }
    /// Provide a reason for the locking of this worktree, if it is locked at all.
    ///
    /// Note that we squelch errors in case the file cannot be read in which case the
    /// reason is an empty string.
    pub fn lock_reason(&self) -> Option<BString> {
        todo!()
    }

    /// Provide a reason
    // TODO: have a custom error type to indicate what when wrong while determining the prunable state.
    //       Potentially add an expiry date here later.
    pub fn is_prunable(&self) -> std::io::Result<()> {
        todo!()
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

    /// Return a list of all _linked_ worktrees sorted by private git dir path as a lightweight proxy.
    ///
    /// Note that these need additional processing to become usable, but provide a first glimpse a typical worktree information.
    pub fn list(&self) -> ! {
        todo!()
    }

    /// Iterate all _linked_ worktrees in sort order and resolve them, ignoring those without an accessible work tree, into repositories
    /// whose [`current()`][worktree::Platform::current()] is the worktree currently being iterated.
    ///
    /// Note that for convenience all io errors are squelched so if there is a chance for IO errors during
    /// traversal of an owned directory, better use `list()` directly. The latter allows to resolve repositories
    /// even if the worktree checkout isn't accessible.
    pub fn iter_repos(&self) -> ! {
        todo!()
    }
}
