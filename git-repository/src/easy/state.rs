#![allow(missing_docs)]
use std::cell::{Ref, RefMut};

use git_ref::file;

use crate::{
    easy,
    easy::{borrow, PackCache},
};

impl Clone for easy::State {
    fn clone(&self) -> Self {
        easy::State { ..Default::default() }
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
    pub(crate) fn try_borrow_mut_pack_cache(&self) -> Result<RefMut<'_, PackCache>, borrow::state::Error> {
        self.pack_cache.try_borrow_mut().map_err(Into::into)
    }

    #[inline]
    pub(crate) fn try_borrow_mut_buf(&self) -> Result<RefMut<'_, Vec<u8>>, borrow::state::Error> {
        self.buf.try_borrow_mut().map_err(Into::into)
    }

    #[inline]
    pub(crate) fn try_borrow_buf(&self) -> Result<Ref<'_, Vec<u8>>, borrow::state::Error> {
        self.buf.try_borrow().map_err(Into::into)
    }
}
