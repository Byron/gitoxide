use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use gix_features::progress::{Count, DynNestedProgress, Progress};

use crate::{loose::Store, Write};

///
pub mod integrity {
    /// The error returned by [`verify_integrity()`][super::Store::verify_integrity()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("{kind} object {id} could not be decoded")]
        ObjectDecode {
            source: gix_object::decode::Error,
            kind: gix_object::Kind,
            id: gix_hash::ObjectId,
        },
        #[error("{kind} object {expected} wasn't re-encoded without change - new hash is {actual}")]
        ObjectHashMismatch {
            kind: gix_object::Kind,
            actual: gix_hash::ObjectId,
            expected: gix_hash::ObjectId,
        },
        #[error("Objects were deleted during iteration - try again")]
        Retry,
        #[error("Interrupted")]
        Interrupted,
    }

    /// The outcome returned by [`verify_integrity()`][super::Store::verify_integrity()].
    #[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Statistics {
        /// The amount of loose objects we checked.
        pub num_objects: usize,
    }

    /// The progress ids used in [`verify_integrity()`][super::Store::verify_integrity()].
    ///
    /// Use this information to selectively extract the progress of interest in case the parent application has custom visualization.
    #[derive(Debug, Copy, Clone)]
    pub enum ProgressId {
        /// The amount of loose objects that have been verified.
        LooseObjects,
    }

    impl From<ProgressId> for gix_features::progress::Id {
        fn from(v: ProgressId) -> Self {
            match v {
                ProgressId::LooseObjects => *b"VILO",
            }
        }
    }
}

impl Store {
    /// Check all loose objects for their integrity checking their hash matches the actual data and by decoding them fully.
    pub fn verify_integrity(
        &self,
        progress: &mut dyn DynNestedProgress,
        should_interrupt: &AtomicBool,
    ) -> Result<integrity::Statistics, integrity::Error> {
        let mut buf = Vec::new();
        let sink = crate::sink(self.object_hash);

        let mut num_objects = 0;
        let start = Instant::now();
        let mut progress = progress.add_child_with_id("Validating".into(), integrity::ProgressId::LooseObjects.into());
        progress.init(None, gix_features::progress::count("loose objects"));
        for id in self.iter().filter_map(Result::ok) {
            let object = self
                .try_find(&id, &mut buf)
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
        progress.show_throughput(start);

        Ok(integrity::Statistics { num_objects })
    }
}
