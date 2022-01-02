use crate::loose::Store;
use crate::Write;

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
        ObjectHashMismatch {
            kind: git_object::Kind,
            actual: git_hash::ObjectId,
            expected: git_hash::ObjectId,
        },
        #[error("Objects were deleted during iteration - try again")]
        Retry,
    }

    /// The outcome returned by [`verify_integrity()`][super::Store::verify_integrity()].
    pub struct Outcome {
        /// The amount of loose objects we checked.
        pub num_objects: usize,
    }
}

impl Store {
    /// Check all loose objects for their integrity checking their hash matches the actual data and by decoding them fully.
    pub fn verify_integrity(&self) -> Result<integrity::Outcome, integrity::Error> {
        let mut buf = Vec::new();
        let mut num_objects = 0;
        let sink = crate::sink(self.object_hash);

        for id in self.iter().filter_map(Result::ok) {
            let object = self
                .try_find(id, &mut buf)
                .map_err(|_| integrity::Error::Retry)?
                .ok_or(integrity::Error::Retry)?;
            let actual_id = sink.write_buf(object.kind, object.data).expect("sink never fails");
            if actual_id != id {
                return Err(integrity::Error::ObjectHashMismatch {
                    kind: object.kind,
                    actual: actual_id,
                    expected: id,
                });
            }
            object.decode().map_err(|err| integrity::Error::ObjectDecode {
                source: err,
                kind: object.kind,
                id,
            })?;
            num_objects += 1;
        }
        Ok(integrity::Outcome { num_objects })
    }
}
