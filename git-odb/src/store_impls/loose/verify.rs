#![allow(missing_docs)]
use crate::loose::Store;

///
pub mod integrity {
    /// The error returned by [`verify_integrity()`][super::Store::verify_integrity()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("{kind} object {id} could not be decoded")]
        ObjectDecode {
            source: git_object::decode::Error,
            kind: git_object::Kind,
            id: git_hash::ObjectId,
        },
        #[error("{kind} object {expected} wasn't re-encoded without change - new hash is {actual}")]
        ObjectEncodeMismatch {
            kind: git_object::Kind,
            actual: git_hash::ObjectId,
            expected: git_hash::ObjectId,
        },
    }

    pub struct Outcome {
        /// The amount of loose objects we checked.
        pub num_objects: usize,
    }
}

impl Store {
    pub fn verify_integrity(&self) -> Result<integrity::Outcome, integrity::Error> {
        todo!()
    }
}
