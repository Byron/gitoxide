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

mod attributes;
mod cache;
mod config;
mod excludes;
mod graph;
pub(crate) mod identity;
mod impls;
mod init;
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
