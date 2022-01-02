use crate::loose::Store;
use crate::Write;
use git_features::progress::Progress;
use std::sync::atomic::{AtomicBool, Ordering};

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
        #[error("Interrupted")]
        Interrupted,
    }

    /// The outcome returned by [`verify_integrity()`][super::Store::verify_integrity()].
    #[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Statistics {
        /// The amount of loose objects we checked.
        pub num_objects: usize,
    }
}

impl Store {
    /// Check all loose objects for their integrity checking their hash matches the actual data and by decoding them fully.
    pub fn verify_integrity(
        &self,
        mut progress: impl Progress,
        should_interrupt: &AtomicBool,
    ) -> Result<integrity::Statistics, integrity::Error> {
        let mut buf = Vec::new();
        let sink = crate::sink(self.object_hash);

        let mut num_objects = 0;
        let mut progress = progress.add_child("validating");
        progress.init(None, git_features::progress::count("objects"));
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

            progress.inc();
            num_objects += 1;
            if should_interrupt.load(Ordering::SeqCst) {
                return Err(integrity::Error::Interrupted);
            }
        }

        Ok(integrity::Statistics { num_objects })
    }
}
