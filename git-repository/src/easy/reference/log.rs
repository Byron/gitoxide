use std::{borrow::Borrow, cell::RefMut, marker::PhantomData, ops::DerefMut};

use git_ref::file::ReferenceExt;

use crate::{easy, easy::Reference};

pub struct Buffer<'repo, A: 'repo, R>
where
    R: Borrow<Reference<'repo, A>>,
{
    pub(crate) reference: R,
    pub(crate) buf: RefMut<'repo, Vec<u8>>,
    pub(crate) _phantom: PhantomData<A>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    BorrowRepo(#[from] easy::borrow::repo::Error),
}

pub type ReverseIter<'a> = git_ref::file::log::iter::Reverse<'a, std::fs::File>;
pub type ForwardIter<'a> = git_ref::file::log::iter::Forward<'a>;

impl<'repo, A, R> Buffer<'repo, A, R>
where
    A: easy::Access + Sized,
    R: Borrow<Reference<'repo, A>>,
{
    pub fn iter_rev(&mut self) -> Result<Option<ReverseIter<'_>>, Error> {
        let buf = self.buf.deref_mut();
        buf.resize(512, 0);
        Ok(self
            .reference
            .borrow()
            .inner
            .log_iter_rev(&self.reference.borrow().access.repo()?.refs, buf)?)
    }

    // TODO: tests
    pub fn iter(&mut self) -> Result<Option<ForwardIter<'_>>, Error> {
        let buf = self.buf.deref_mut();
        Ok(self
            .reference
            .borrow()
            .inner
            .log_iter(&self.reference.borrow().access.repo()?.refs, buf)?)
    }
}

impl<'repo, A> Reference<'repo, A>
where
    A: easy::Access + Sized,
{
    pub fn log(&self) -> Result<Buffer<'repo, A, &'_ Reference<'repo, A>>, easy::borrow::state::Error> {
        Ok(Buffer {
            reference: self,
            buf: self.access.state().try_borrow_mut_buf()?,
            _phantom: Default::default(),
        })
    }
}
