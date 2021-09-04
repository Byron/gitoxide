use std::{cell::RefMut, ops::DerefMut};

use git_ref::file::ReferenceExt;

use crate::{easy, easy::Reference};

pub struct Buffer<'a, 'repo, A> {
    reference: &'a Reference<'repo, A>,
    buf: RefMut<'repo, Vec<u8>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    BorrowRepo(#[from] easy::borrow::repo::Error),
}

impl<'a, 'repo, A> Buffer<'a, 'repo, A>
where
    A: easy::Access + Sized,
{
    pub fn reverse_iter(&mut self) -> Result<Option<git_ref::file::log::iter::Reverse<'_, std::fs::File>>, Error> {
        let buf = self.buf.deref_mut();
        buf.resize(512, 0);
        Ok(self
            .reference
            .inner
            .log_iter_rev(&self.reference.access.repo()?.refs, buf)?)
    }
}

impl<'repo, A> Reference<'repo, A>
where
    A: easy::Access + Sized,
{
    pub fn log(&self) -> Result<Buffer<'_, 'repo, A>, easy::borrow::state::Error> {
        Ok(Buffer {
            reference: self,
            buf: self.access.state().try_borrow_mut_buf()?,
        })
    }
}
