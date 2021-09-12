use std::{
    cell::{BorrowError, BorrowMutError},
    time::SystemTime,
};

use git_ref::file;

use crate::easy;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PackedRefsOpen(#[from] git_ref::packed::buffer::open::Error),
    #[error("BUG: Part of interior state could not be borrowed.")]
    BorrowState(#[from] easy::borrow::state::Error),
}

impl From<std::cell::BorrowError> for Error {
    fn from(err: BorrowError) -> Self {
        Error::BorrowState(easy::borrow::state::Error::Borrow(err))
    }
}

impl From<std::cell::BorrowMutError> for Error {
    fn from(err: BorrowMutError) -> Self {
        Error::BorrowState(easy::borrow::state::Error::BorrowMut(err))
    }
}

#[derive(Default)]
pub(crate) struct ModifieablePackedRefsBuffer {
    pub(crate) packed_refs: Option<git_ref::packed::Buffer>,
    modified: Option<SystemTime>,
}

impl ModifieablePackedRefsBuffer {
    pub fn assure_packed_refs_uptodate(
        &mut self,
        file: &file::Store,
    ) -> Result<(), git_ref::packed::buffer::open::Error> {
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
