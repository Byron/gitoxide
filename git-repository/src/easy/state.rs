use std::{
    cell::{Ref, RefMut},
    ops::DerefMut,
};

use crate::{
    easy,
    easy::{borrow, PackCache},
    refs,
    refs::{file, packed},
};

impl easy::State {
    // TODO: this method should be on the Store itself, as one day there will be reftable support which lacks packed-refs
    // TODO: provide a way to update a cache if the underlying pack changed or got deleted.
    pub(crate) fn assure_packed_refs_present(&self, file: &file::Store) -> Result<(), packed::buffer::open::Error> {
        if self.packed_refs.borrow().is_none() {
            *self.packed_refs.borrow_mut().deref_mut() = file.packed_buffer()?;
        }
        Ok(())
    }

    #[inline]
    pub(crate) fn try_borrow_packed_refs(&self) -> Result<Ref<'_, Option<refs::packed::Buffer>>, borrow::state::Error> {
        self.packed_refs.try_borrow().map_err(Into::into)
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
