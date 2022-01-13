//!
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
};

use crate::easy;

impl Clone for easy::Handle {
    fn clone(&self) -> Self {
        easy::Handle::from_refs_and_objects(
            self.refs.clone(),
            self.objects.clone(),
            self.object_hash,
            self.work_tree.clone(),
        )
    }
}

/// Access
impl easy::Handle {
    /// Return the work tree containing all checked out files, if there is one.
    pub fn work_tree(&self) -> Option<&Path> {
        self.work_tree.as_deref()
    }
}

impl easy::Handle {
    pub(crate) fn from_refs_and_objects(
        refs: crate::RefStore,
        objects: crate::OdbHandle,
        object_hash: git_hash::Kind,
        work_tree: Option<PathBuf>,
    ) -> Self {
        easy::Handle {
            bufs: RefCell::new(Vec::with_capacity(4)),
            object_hash,
            work_tree,
            objects: {
                #[cfg(feature = "max-performance")]
                {
                    objects.with_pack_cache(|| Box::new(git_pack::cache::lru::StaticLinkedList::<64>::default()))
                }
                #[cfg(not(feature = "max-performance"))]
                {
                    objects
                }
            },
            refs,
        }
    }
}

impl From<&crate::Repository> for easy::Handle {
    fn from(repo: &crate::Repository) -> Self {
        easy::Handle::from_refs_and_objects(
            repo.refs.clone(),
            repo.objects.to_handle().into(),
            repo.object_hash,
            repo.work_tree.clone(),
        )
    }
}

impl From<crate::Repository> for easy::Handle {
    fn from(repo: crate::Repository) -> Self {
        (&repo).into()
    }
}

impl easy::Handle {
    #[inline]
    pub(crate) fn free_buf(&self) -> Vec<u8> {
        self.bufs.borrow_mut().pop().unwrap_or_default()
    }

    #[inline]
    pub(crate) fn reuse_buffer(&self, data: &mut Vec<u8>) {
        if data.capacity() > 0 {
            self.bufs.borrow_mut().push(std::mem::take(data));
        }
    }
}
