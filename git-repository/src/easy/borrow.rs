//!
///
pub mod state {
    /// An error to indicate that `Easy*` state could not be borrowed mutable or immutably.
    ///
    /// With `Easy*`borrowing rules are validated at runtime, resulting in additional failure modes.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("A state member could not be borrowed")]
        Borrow(#[from] std::cell::BorrowError),
        #[error("A state member could not be mutably borrowed")]
        BorrowMut(#[from] std::cell::BorrowMutError),
    }

    /// A utility type to help with state accessors that may fail due to runtime borrow checking.
    pub type Result<T> = std::result::Result<T, Error>;
}

///
pub mod repo {
    use std::{
        cell::{BorrowError, BorrowMutError},
        fmt::{Display, Formatter},
    };

    /// An error to indicate that the [`Repository`][crate::Repository] could not be borrowed immutably or mutably as
    /// another mutable borrowed is already present.
    ///
    /// Note that mutable borrows are uncommon and used only in rare cases, and with [`EasyArcExclusive`][crate::EasyArcExclusive]
    /// errors of this kind can't happen as a lock would block instead.
    #[derive(Debug)]
    pub struct Error;

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            "Could not borrow the repository as it's already borrowed mutably".fmt(f)
        }
    }

    impl std::error::Error for Error {}

    impl From<BorrowError> for Error {
        fn from(_: BorrowError) -> Self {
            Error
        }
    }

    impl From<BorrowMutError> for Error {
        fn from(_: BorrowMutError) -> Self {
            Error
        }
    }

    /// A utility type to help with repository accessors that may fail due to runtime borrow checking.
    pub type Result<T> = std::result::Result<T, Error>;
}
