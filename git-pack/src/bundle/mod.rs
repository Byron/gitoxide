///
pub mod init;

mod find;
///
pub mod write;

///
pub mod verify {
    use std::sync::atomic::AtomicBool;

    use git_features::progress::Progress;

    ///
    pub mod integrity {
        /// Returned by [`Bundle::verify_integrity()`][crate::Bundle::verify_integrity()].
        pub struct Outcome<P> {
            /// The computed checksum of the index which matched the stored one.
            pub actual_index_checksum: git_hash::ObjectId,
            /// The packs traversal outcome
            pub pack_traverse_outcome: crate::index::traverse::Outcome,
            /// The provided progress instance.
            pub progress: P,
        }
    }

    use crate::Bundle;

    impl Bundle {
        /// Similar to [`crate::index::File::verify_integrity()`] but more convenient to call as the presence of the
        /// pack file is a given.
        pub fn verify_integrity<C, P>(
            &self,
            verify_mode: crate::index::verify::Mode,
            traversal: crate::index::traverse::Algorithm,
            make_pack_lookup_cache: impl Fn() -> C + Send + Clone,
            thread_limit: Option<usize>,
            progress: P,
            should_interrupt: &AtomicBool,
        ) -> Result<integrity::Outcome<P>, crate::index::traverse::Error<crate::index::verify::integrity::Error>>
        where
            P: Progress,
            C: crate::cache::DecodeEntry,
        {
            self.index
                .verify_integrity(
                    Some(crate::index::verify::PackContext {
                        data: &self.pack,
                        verify_mode,
                        traversal_algorithm: traversal,
                        make_cache_fn: make_pack_lookup_cache,
                    }),
                    thread_limit,
                    progress,
                    should_interrupt,
                )
                .map(|o| integrity::Outcome {
                    actual_index_checksum: o.actual_index_checksum,
                    pack_traverse_outcome: o.pack_traverse_outcome.expect("pack is set"),
                    progress: o.progress,
                })
        }
    }
}
