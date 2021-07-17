use crate::{mutable::Target, PartialName};

/// A minimal trait to group useful operations for handling references across store implementations.
pub trait RefStore {
    /// The error used in [`RefStore::find_existing()`].
    type FindExistingError;

    /// Find the reference with the given `name`. Return `Ok(None)` if the reference doesn't exist.
    fn find_existing(&self, name: PartialName<'_>) -> Result<Target, Self::FindExistingError>;
}
