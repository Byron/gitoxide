//!
use std::cell::{Ref, RefCell, RefMut};

use crate::{easy, easy::borrow};

impl Clone for easy::Handle {
    fn clone(&self) -> Self {
        easy::Handle::from_refs_and_objects(self.refs.clone(), self.objects.clone(), self.hash_kind)
    }
}

impl easy::Handle {
    pub(crate) fn from_refs_and_objects(
        refs: crate::RefStore,
        objects: crate::OdbHandle,
        hash_kind: git_hash::Kind,
    ) -> Self {
        easy::Handle {
            buf: RefCell::new(vec![]),
            hash_kind,
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
        easy::Handle::from_refs_and_objects(repo.refs.clone(), repo.objects.to_handle_shared(), repo.hash_kind)
    }
}

impl easy::Handle {
    #[inline]
    pub(crate) fn try_borrow_mut_buf(&self) -> borrow::state::Result<RefMut<'_, Vec<u8>>> {
        self.buf.try_borrow_mut().map_err(Into::into)
    }

    #[inline]
    pub(crate) fn try_borrow_buf(&self) -> borrow::state::Result<Ref<'_, Vec<u8>>> {
        self.buf.try_borrow().map_err(Into::into)
    }
}
