use crate::Repository;
#[cfg(all(feature = "unstable", feature = "git-worktree"))]
pub use git_worktree::*;

///
#[cfg(feature = "git-index")]
pub mod open_index {
    use crate::bstr::BString;

    /// The error returned by [`Worktree::open_index()`][crate::Worktree::open_index()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not interpret value '{}' as 'index.threads'", .value)]
        ConfigIndexThreads {
            value: BString,
            #[source]
            err: git_config::value::parse::Error,
        },
        #[error(transparent)]
        IndexFile(#[from] git_index::file::init::Error),
    }
}

/// A structure to make the API more stuctured.
pub struct Platform<'repo> {
    pub(crate) parent: &'repo Repository,
}

impl<'repo> crate::Worktree<'repo> {
    /// Open a new copy of the index file and decode it entirely.
    ///
    /// It will use the `index.threads` configuration key to learn how many threads to use.
    // TODO: test
    #[cfg(feature = "git-index")]
    pub fn open_index(&self) -> Result<git_index::File, crate::worktree::open_index::Error> {
        use std::convert::{TryFrom, TryInto};
        let repo = self.parent;
        let thread_limit = repo
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
