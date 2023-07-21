//!

/// The kind of repository.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    /// A submodule worktree, whose `git` repository lives in `.git/modules/**/<name>` of the parent repository.
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

pub mod attributes;
mod cache;
mod config;
mod excludes;
///
pub mod filter;
mod graph;
pub(crate) mod identity;
mod impls;
mod index;
pub(crate) mod init;
mod kind;
mod location;
mod object;
mod reference;
mod remote;
mod revision;
mod shallow;
mod snapshots;
mod state;
mod thread_safe;
mod worktree;

/// A type to represent an index which either was loaded from disk as it was persisted there, or created on the fly in memory.
pub enum IndexPersistedOrInMemory {
    /// The index as loaded from disk, and shared across clones of the owning `Repository`.
    Persisted(crate::worktree::Index),
    /// A temporary index as created from the `HEAD^{tree}`, with the file path set to the place where it would be stored naturally.
    ///
    /// Note that unless saved explicitly, it will not persist.
    InMemory(gix_index::File),
}

///
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
        TraverseTree(#[from] gix_traverse::tree::breadthfirst::Error),
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
        OpenTree(#[from] gix_traverse::tree::breadthfirst::Error),
        #[error(transparent)]
        AttributesCache(#[from] crate::repository::attributes::Error),
        #[error(transparent)]
        FilterPipeline(#[from] crate::filter::pipeline::options::Error),
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
