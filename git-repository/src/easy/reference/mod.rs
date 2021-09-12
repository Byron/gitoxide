//!
use std::ops::DerefMut;

use git_odb::Find;
use git_ref::file::ReferenceExt;

use crate::{
    easy,
    easy::{Oid, Reference},
};

pub mod iter;

impl<'repo, A> Reference<'repo, A> {
    /// Return the target to which this reference points to.
    pub fn target(&self) -> git_ref::TargetRef<'_> {
        self.inner.target.to_ref()
    }

    /// Return the reference's full name.
    pub fn name(&self) -> git_ref::FullNameRef<'_> {
        self.inner.name.to_ref()
    }

    /// Turn this instances into a stand-alone reference.
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

    /// Follow all symbolic targets this reference might point to and peel the underlying object
    /// to the end of the chain, and return it.
    ///
    /// This is useful to learn where this reference is ulitmately pointing to.
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

    /// Similar to [`peel_to_id_in_place()`][Reference::peel_to_id_in_place()], but consumes this instance.
    pub fn into_fully_peeled_id(mut self) -> Result<Oid<'repo, A>, peel::Error> {
        self.peel_to_id_in_place()
    }
}

///
pub mod namespace {
    ///
    pub mod set {
        use crate::easy;

        /// The error returned by [ReferenceAccessExt::set_namespace(…)][easy::ext::ReferenceAccessExt::set_namespace()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            BorrowRepoMut(#[from] easy::borrow::repo::Error),
            #[error(transparent)]
            NameValidation(#[from] git_validate::refname::Error),
        }
    }
}

///
pub mod edit {
    use crate::easy;

    /// The error returned by [ReferenceAccessExt::edit_references(…)][easy::ext::ReferenceAccessExt::edit_references()], and others
    /// which ultimately create a reference.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
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

///
pub mod peel {
    use crate::easy;

    /// The error returned by [Reference::peel_to_id_in_place(…)][easy::Reference::peel_to_id_in_place()] and
    /// [Reference::into_fully_peeled_id(…)][easy::Reference::into_fully_peeled_id()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
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

pub mod log;
pub(crate) mod packed;

///
pub mod find {
    use crate::easy;

    ///
    pub mod existing {
        use crate::easy;

        /// The error returned by [ReferenceAccessExt::find_reference(…)][easy::ext::ReferenceAccessExt::find_reference()], and others.
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            Find(#[from] easy::reference::find::Error),
            #[error("The reference did not exist even though that was expected")]
            NotFound,
        }
    }

    /// The error returned by [ReferenceAccessExt::try_find_reference(…)][easy::ext::ReferenceAccessExt::try_find_reference()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
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
