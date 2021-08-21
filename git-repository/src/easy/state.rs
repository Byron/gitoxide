use std::cell::{Ref, RefMut};

use crate::{
    easy,
    easy::{borrow, PackCache},
    refs::{file, packed},
};

impl Clone for easy::State {
    fn clone(&self) -> Self {
        easy::State {
            packed_refs: self.packed_refs.clone(),
            ..Default::default()
        }
    }
}

impl easy::ModifieablePackedRefsBuffer {
    fn assure_packed_refs_uptodate(&mut self, file: &file::Store) -> Result<(), packed::buffer::open::Error> {
        let packed_refs_modified_time = || file.packed_refs_path().metadata().and_then(|m| m.modified()).ok();
        if self.packed_refs.is_none() {
            self.packed_refs = file.packed_buffer()?;
            if self.packed_refs.is_some() {
                self.modified = packed_refs_modified_time();
            }
        } else {
            let recent_modification = packed_refs_modified_time();
            match (&self.modified, recent_modification) {
                (None, None) => {}
                (Some(_), None) => {
                    self.packed_refs = None;
                    self.modified = None
                }
                (Some(cached_time), Some(modified_time)) => {
                    if *cached_time < modified_time {
                        self.packed_refs = file.packed_buffer()?;
                        self.modified = Some(modified_time);
                    }
                }
                (None, Some(modified_time)) => {
                    self.packed_refs = file.packed_buffer()?;
                    self.modified = Some(modified_time);
                }
            }
        }
        Ok(())
    }
}

impl easy::State {
    // TODO: this method should be on the Store itself, as one day there will be reftable support which lacks packed-refs
    pub(crate) fn assure_packed_refs_uptodate(
        &self,
        file: &file::Store,
    ) -> Result<parking_lot::MappedRwLockReadGuard<'_, Option<packed::Buffer>>, packed::buffer::open::Error> {
        let mut packed_refs = self.packed_refs.write();
        packed_refs.assure_packed_refs_uptodate(file)?;
        let packed_refs = parking_lot::RwLockWriteGuard::<'_, _>::downgrade(packed_refs);
        Ok(parking_lot::RwLockReadGuard::<'_, _>::map(packed_refs, |buffer| {
            &buffer.packed_refs
        }))
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
