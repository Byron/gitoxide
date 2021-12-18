///
pub mod init;

mod find;
///
pub mod write;

mod verify {
    use std::sync::{atomic::AtomicBool, Arc};

    use git_features::progress::Progress;

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
            progress: Option<P>,
            should_interrupt: Arc<AtomicBool>,
        ) -> Result<
            (git_hash::ObjectId, Option<crate::index::traverse::Outcome>, Option<P>),
            crate::index::traverse::Error<crate::index::verify::Error>,
        >
        where
            P: Progress,
            C: crate::cache::DecodeEntry,
        {
            self.index.verify_integrity(
                Some((&self.pack, verify_mode, traversal, make_pack_lookup_cache)),
                thread_limit,
                progress,
                should_interrupt,
            )
        }
    }
}
