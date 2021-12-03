//!
use std::cell::{Ref, RefCell, RefMut};

use crate::{easy, easy::borrow};

impl Clone for easy::State {
    fn clone(&self) -> Self {
        easy::State::from_refs_and_objects(self.refs.clone(), self.objects.clone())
    }
}

impl easy::State {
    pub(crate) fn from_refs_and_objects(refs: crate::RefStore, objects: crate::OdbHandle) -> Self {
        easy::State {
            #[cfg(not(feature = "max-performance"))]
            pack_cache: RefCell::new(git_pack::cache::Never),
            #[cfg(feature = "max-performance")]
            pack_cache: RefCell::new(Box::new(git_pack::cache::lru::StaticLinkedList::<64>::default())),
            object_cache: RefCell::new(None),
            buf: RefCell::new(vec![]),
            objects,
            refs,
        }
    }
}

impl From<&crate::Repository> for easy::State {
    fn from(repo: &crate::Repository) -> Self {
        easy::State::from_refs_and_objects(repo.refs.clone(), repo.odb.to_handle_shared())
    }
}

impl easy::State {
    #[inline]
    pub(crate) fn try_borrow_mut_pack_cache(&self) -> borrow::state::Result<RefMut<'_, easy::PackCache>> {
        self.pack_cache.try_borrow_mut().map_err(Into::into)
    }

    #[inline]
    pub(crate) fn try_borrow_mut_object_cache(
        &self,
    ) -> borrow::state::Result<RefMut<'_, Option<easy::object::cache::MemoryCappedHashmap>>> {
        self.object_cache.try_borrow_mut().map_err(Into::into)
    }

    #[inline]
    pub(crate) fn try_borrow_mut_buf(&self) -> borrow::state::Result<RefMut<'_, Vec<u8>>> {
        self.buf.try_borrow_mut().map_err(Into::into)
    }

    #[inline]
    pub(crate) fn try_borrow_buf(&self) -> borrow::state::Result<Ref<'_, Vec<u8>>> {
        self.buf.try_borrow().map_err(Into::into)
    }
}
