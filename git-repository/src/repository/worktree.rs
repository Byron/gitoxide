use crate::{worktree, Worktree};
use std::convert::{TryFrom, TryInto};

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

impl<'repo> Worktree<'repo> {
    /// Open a new copy of the index file and decode it entirely.
    ///
    /// It will use the `index.threads` configuration key to learn how many threads to use.
    #[cfg(feature = "git-index")]
    pub fn open_index(&self) -> Result<git_index::File, crate::worktree::open_index::Error> {
        let repo = self.parent;
        let thread_limit = repo
            .config
            .resolved
            .boolean("index", None, "threads")
            .map(|res| {
                res.map(|value| if value { 0usize } else { 1 }).or_else(|err| {
                    git_config::values::Integer::try_from(err.input.as_ref())
                        .map_err(|_err| crate::worktree::open_index::Error::ConfigIndexThreads {
                            value: repo
                                .config
                                .resolved
                                .string("core", None, "threads")
                                .expect("present")
                                .into_owned(),
                        })
                        .map(|value| value.to_decimal().and_then(|v| v.try_into().ok()).unwrap_or(1))
                })
            })
            .transpose()?;
        git_index::File::at(
            repo.git_dir().join("index"),
            git_index::decode::Options {
                object_hash: repo.object_hash(),
                thread_limit,
                min_extension_block_in_bytes_for_threading: 0,
            },
        )
        .map_err(Into::into)
    }
}
