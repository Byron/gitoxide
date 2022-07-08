/// The error when looking up a value.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error<E> {
    #[error(transparent)]
    ValueMissing(#[from] crate::lookup::existing::Error),
    #[error(transparent)]
    FailedConversion(E),
}

///
pub mod existing {
    /// The error when looking up a value that doesn't exist.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The requested section does not exist")]
        SectionMissing,
        #[error("The requested subsection does not exist")]
        SubSectionMissing,
        #[error("The key does not exist in the requested section")]
        KeyMissing,
    }
}
