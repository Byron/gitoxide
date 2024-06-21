//!
#![allow(clippy::empty_docs)]

/// The kind of repository.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    /// A submodule worktree, whose `git` repository lives in `.git/modules/**/<name>` of the parent repository.
    ///
    /// Note that 'old-form' submodule will register as `Worktree {is_linked: false}`.
    Submodule,
    /// A bare repository does not have a work tree, that is files on disk beyond the `git` repository itself.
    Bare,
    /// A `git` repository along with a checked out files in a work tree.
    WorkTree {
        /// If true, this is the git dir associated with this _linked_ worktree, otherwise it is a repository with _main_ worktree.
        is_linked: bool,
    },
}

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

#[cfg(any(feature = "attributes", feature = "excludes"))]
pub mod attributes;
mod cache;
mod config;
///
#[allow(clippy::empty_docs)]
#[cfg(feature = "blob-diff")]
pub mod diff;
///
#[allow(clippy::empty_docs)]
#[cfg(feature = "dirwalk")]
mod dirwalk;
///
#[allow(clippy::empty_docs)]
#[cfg(feature = "attributes")]
pub mod filter;
mod graph;
pub(crate) mod identity;
mod impls;
#[cfg(feature = "index")]
mod index;
pub(crate) mod init;
mod kind;
mod location;
#[cfg(feature = "mailmap")]
mod mailmap;
mod object;
#[cfg(feature = "attributes")]
mod pathspec;
mod reference;
mod remote;
#[cfg(feature = "revision")]
mod revision;
mod shallow;
mod state;
#[cfg(feature = "attributes")]
mod submodule;
mod thread_safe;
mod worktree;

///
#[allow(clippy::empty_docs)]
#[cfg(feature = "index")]
pub mod index_from_tree {
    /// The error returned by [Repository::index_from_tree()](crate::Repository::index_from_tree).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not create index from tree at {id}")]
        IndexFromTree {
            id: gix_hash::ObjectId,
            source: gix_index::init::from_tree::Error,
        },
        #[error("Couldn't obtain configuration for core.protect*")]
        BooleanConfig(#[from] crate::config::boolean::Error),
    }
}

///
#[allow(clippy::empty_docs)]
pub mod branch_remote_ref_name {

    /// The error returned by [Repository::branch_remote_ref_name()](crate::Repository::branch_remote_ref_name()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The configured name of the remote ref to merge wasn't valid")]
        ValidateFetchRemoteRefName(#[from] gix_validate::reference::name::Error),
        #[error(transparent)]
        PushDefault(#[from] crate::config::key::GenericErrorWithValue),
        #[error(transparent)]
        FindPushRemote(#[from] crate::remote::find::existing::Error),
    }
}

///
#[allow(clippy::empty_docs)]
pub mod branch_remote_tracking_ref_name {

    /// The error returned by [Repository::branch_remote_tracking_ref_name()](crate::Repository::branch_remote_tracking_ref_name()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The name of the tracking reference was invalid")]
        ValidateTrackingRef(#[from] gix_validate::reference::name::Error),
        #[error("Could not get the remote reference to translate into the local tracking branch")]
        RemoteRef(#[from] super::branch_remote_ref_name::Error),
        #[error("Couldn't find remote to obtain fetch-specs for mapping to the tracking reference")]
        FindRemote(#[from] crate::remote::find::existing::Error),
    }
}

///
#[cfg(feature = "attributes")]
pub mod pathspec_defaults_ignore_case {
    /// The error returned by [Repository::pathspec_defaults_ignore_case()](crate::Repository::pathspec_defaults_inherit_ignore_case()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Filesystem configuration could not be obtained to learn about case sensitivity")]
        FilesystemConfig(#[from] crate::config::boolean::Error),
        #[error(transparent)]
        Defaults(#[from] gix_pathspec::defaults::from_environment::Error),
    }
}

///
#[cfg(feature = "index")]
pub mod index_or_load_from_head {
    /// The error returned by [`Repository::index_or_load_from_head()`][crate::Repository::index_or_load_from_head()].
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        HeadCommit(#[from] crate::reference::head_commit::Error),
        #[error(transparent)]
        TreeId(#[from] gix_object::decode::Error),
        #[error(transparent)]
        TraverseTree(#[from] crate::repository::index_from_tree::Error),
        #[error(transparent)]
        OpenIndex(#[from] crate::worktree::open_index::Error),
    }
}

///
#[cfg(feature = "worktree-stream")]
pub mod worktree_stream {
    /// The error returned by [`Repository::worktree_stream()`][crate::Repository::worktree_stream()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        FindTree(#[from] crate::object::find::existing::Error),
        #[error(transparent)]
        OpenTree(#[from] crate::repository::index_from_tree::Error),
        #[error(transparent)]
        AttributesCache(#[from] crate::config::attribute_stack::Error),
        #[error(transparent)]
        FilterPipeline(#[from] crate::filter::pipeline::options::Error),
        #[error(transparent)]
        CommandContext(#[from] crate::config::command_context::Error),
        #[error("Needed {id} to be a tree to turn into a workspace stream, got {actual}")]
        NotATree {
            id: gix_hash::ObjectId,
            actual: gix_object::Kind,
        },
    }
}

///
#[cfg(feature = "worktree-archive")]
pub mod worktree_archive {
    /// The error returned by [`Repository::worktree_archive()`][crate::Repository::worktree_archive()].
    pub type Error = gix_archive::Error;
}
