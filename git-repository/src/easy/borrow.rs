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
