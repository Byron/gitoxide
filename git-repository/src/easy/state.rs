//!
use std::cell::{Ref, RefCell, RefMut};

use git_ref::file;

use crate::{easy, easy::borrow};

impl Clone for easy::State {
    fn clone(&self) -> Self {
        easy::State { ..Default::default() }
    }
}

impl Default for easy::State {
    fn default() -> Self {
        easy::State {
            packed_refs: RefCell::new(Default::default()),
            #[cfg(not(feature = "max-performance"))]
            pack_cache: RefCell::new(git_pack::cache::Never),
            #[cfg(feature = "max-performance")]
            pack_cache: RefCell::new(Box::new(git_pack::cache::lru::StaticLinkedList::<64>::default())),
            object_cache: RefCell::new(None),
            buf: RefCell::new(vec![]),
        }
    }
}

impl easy::State {
    pub(crate) fn assure_packed_refs_uptodate(
        &self,
        file: &file::Store,
    ) -> Result<Ref<'_, easy::reference::packed::ModifieablePackedRefsBuffer>, easy::reference::packed::Error> {
        let mut packed_refs = self.packed_refs.try_borrow_mut()?;
        packed_refs.assure_packed_refs_uptodate(file)?;
        drop(packed_refs);
        Ok(self.packed_refs.try_borrow()?)
    }

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
