#![allow(missing_docs)]
use std::ops::DerefMut;

use git_odb::Find;

use crate::{
    easy,
    easy::{Oid, Reference},
};

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

pub mod peel_to_oid_in_place {
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
}

impl<'repo, A> Reference<'repo, A>
where
    A: easy::Access + Sized,
{
    pub(crate) fn from_file_ref(reference: git_ref::Reference, access: &'repo A) -> Self {
        Reference {
            inner: reference,
            access,
        }
    }
    pub fn target(&self) -> git_ref::TargetRef<'_> {
        self.inner.target.to_ref()
    }

    pub fn name(&self) -> git_ref::FullNameRef<'_> {
        self.inner.name.to_ref()
    }

    pub fn detach(self) -> git_ref::Reference {
        self.inner
    }

    pub fn peel_to_oid_in_place(&mut self) -> Result<Oid<'repo, A>, peel_to_oid_in_place::Error> {
        let repo = self.access.repo()?;
        let state = self.access.state();
        let mut pack_cache = state.try_borrow_mut_pack_cache()?;
        let oid = self.inner.peel_to_id_in_place(
            &repo.refs,
            state.assure_packed_refs_uptodate(&repo.refs)?.as_ref(),
            |oid, buf| {
                repo.odb
                    .try_find(oid, buf, pack_cache.deref_mut())
                    .map(|po| po.map(|o| (o.kind, o.data)))
            },
        )?;
        Ok(Oid::from_id(oid, self.access))
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
}
