#![allow(missing_docs)]
use std::ops::DerefMut;

use git_odb::Find;
use git_ref::file::ReferenceExt;

use crate::{
    easy,
    easy::{Oid, Reference},
};

pub mod namespace {
    pub mod set {
        use crate::easy;

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            BorrowRepoMut(#[from] easy::borrow::repo::Error),
            #[error(transparent)]
            NameValidation(#[from] git_validate::refname::Error),
        }
    }
}

pub mod create {
    use crate::easy;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Edit(#[from] easy::reference::edit::Error),
        #[error(transparent)]
        NameValidation(#[from] git_validate::reference::name::Error),
    }
}

pub mod edit {
    use crate::easy;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        FileTransactionPrepare(#[from] git_ref::file::transaction::prepare::Error),
        #[error(transparent)]
        FileTransactionCommit(#[from] git_ref::file::transaction::commit::Error),
        #[error(transparent)]
        NameValidation(#[from] git_validate::reference::name::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }
}

pub mod peel {
    use crate::easy;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        PeelToId(#[from] git_ref::peel::to_id::Error),
        #[error(transparent)]
        PackedRefsOpen(#[from] git_ref::packed::buffer::open::Error),
        #[error("BUG: Part of interior state could not be borrowed.")]
        BorrowState(#[from] easy::borrow::state::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }

    impl From<easy::reference::packed::Error> for Error {
        fn from(err: easy::reference::packed::Error) -> Self {
            match err {
                easy::reference::packed::Error::PackedRefsOpen(err) => Error::PackedRefsOpen(err),
                easy::reference::packed::Error::BorrowState(err) => Error::BorrowState(err),
            }
        }
    }
}

impl<'repo, A> Reference<'repo, A> {
    pub fn target(&self) -> git_ref::TargetRef<'_> {
        self.inner.target.to_ref()
    }

    pub fn name(&self) -> git_ref::FullNameRef<'_> {
        self.inner.name.to_ref()
    }

    pub fn detach(self) -> git_ref::Reference {
        self.inner
    }
}

impl<'repo, A> Reference<'repo, A>
where
    A: easy::Access + Sized,
{
    pub(crate) fn from_ref(reference: git_ref::Reference, access: &'repo A) -> Self {
        Reference {
            inner: reference,
            access,
        }
    }

    pub fn peel_to_id_in_place(&mut self) -> Result<Oid<'repo, A>, peel::Error> {
        let repo = self.access.repo()?;
        let state = self.access.state();
        let mut pack_cache = state.try_borrow_mut_pack_cache()?;
        let oid = self.inner.peel_to_id_in_place(
            &repo.refs,
            state.assure_packed_refs_uptodate(&repo.refs)?.packed_refs.as_ref(),
            |oid, buf| {
                repo.odb
                    .try_find(oid, buf, pack_cache.deref_mut())
                    .map(|po| po.map(|o| (o.kind, o.data)))
            },
        )?;
        Ok(Oid::from_id(oid, self.access))
    }

    pub fn into_fully_peeled_id(mut self) -> Result<Oid<'repo, A>, peel::Error> {
        self.peel_to_id_in_place()
    }
}

pub mod log;

pub(crate) mod packed {
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
}

pub mod find {
    use crate::easy;

    pub mod existing {

        use crate::easy::reference::find;

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            Find(#[from] find::Error),
            #[error("The reference did not exist even though that was expected")]
            NotFound,
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Find(#[from] git_ref::file::find::Error),
        #[error(transparent)]
        PackedRefsOpen(#[from] git_ref::packed::buffer::open::Error),
        #[error("BUG: Part of interior state could not be borrowed.")]
        BorrowState(#[from] easy::borrow::state::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }

    impl From<easy::reference::packed::Error> for Error {
        fn from(err: easy::reference::packed::Error) -> Self {
            match err {
                easy::reference::packed::Error::PackedRefsOpen(err) => Error::PackedRefsOpen(err),
                easy::reference::packed::Error::BorrowState(err) => Error::BorrowState(err),
            }
        }
    }
}
