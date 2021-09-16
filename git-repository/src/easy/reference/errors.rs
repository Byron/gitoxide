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
        ToId(#[from] git_ref::peel::to_id::Error),
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
