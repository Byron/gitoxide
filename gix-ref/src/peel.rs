///
#[allow(clippy::empty_docs)]
pub mod to_id {
    use std::path::PathBuf;

    use gix_object::bstr::BString;

    use crate::file;

    /// The error returned by [`crate::file::ReferenceExt::peel_to_id_in_place()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not follow a single level of a symbolic reference")]
        Follow(#[from] file::find::existing::Error),
        #[error("Aborting due to reference cycle with first seen path being {start_absolute:?}")]
        Cycle { start_absolute: PathBuf },
        #[error("Refusing to follow more than {max_depth} levels of indirection")]
        DepthLimitExceeded { max_depth: usize },
        #[error("An error occurred when trying to resolve an object a reference points to")]
        Find(#[from] gix_object::find::Error),
        #[error("Object {oid} as referred to by {name:?} could not be found")]
        NotFound { oid: gix_hash::ObjectId, name: BString },
    }
}
