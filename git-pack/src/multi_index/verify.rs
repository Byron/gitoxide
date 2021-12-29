use crate::multi_index::File;
use git_features::progress::Progress;
use std::sync::atomic::AtomicBool;

///
pub mod checksum {
    /// Returned by [`index::File::verify_checksum()`][crate::index::File::verify_checksum()].
    pub type Error = crate::verify::checksum::Error;
}

impl File {
    /// Validate that our [`checksum()`][File::checksum()] matches the actual contents
    /// of this index file, and return it if it does.
    pub fn verify_checksum(
        &self,
        progress: impl Progress,
        should_interrupt: &AtomicBool,
    ) -> Result<git_hash::ObjectId, checksum::Error> {
        crate::verify::checksum_on_disk_or_mmap(
            self.path(),
            &self.data,
            self.checksum(),
            self.object_hash,
            progress,
            should_interrupt,
        )
    }

    /// Similar to [`crate::Bundle::verify_integrity()`] but checks all contained indices and their packs.
    ///
    /// Note that it's considered a failure if an index doesn't have a corresponding pack.
    #[allow(unused)]
    pub fn verify_integrity<C, P>(
        &self,
        verify_mode: crate::index::verify::Mode,
        traversal: crate::index::traverse::Algorithm,
        make_pack_lookup_cache: impl Fn() -> C + Send + Clone,
        thread_limit: Option<usize>,
        progress: Option<P>,
        should_interrupt: &AtomicBool,
    ) -> Result<
        (git_hash::ObjectId, Option<crate::index::traverse::Outcome>, Option<P>),
        crate::index::traverse::Error<crate::index::verify::integrity::Error>,
    >
    where
        P: Progress,
        C: crate::cache::DecodeEntry,
    {
        todo!()
    }
}
