/// A way to uniquely identify the location of an object within a pack bundle
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Location {
    /// The id of the pack containing the object //TODO: this should  probably at least by a typedef or even an opaque type
    pub pack_id: u32,
    /// The index at which the object can be found in the index file corresponding to the `pack_id`.
    pub index_file_id: u32,
    /// The size of the entry of disk so that the range of bytes of the entry is `pack_offset..pack_offset + entry_size`.
    pub entry_size: usize,
    /// The start of the entry in the pack identified by `pack_id`.
    pub pack_offset: u64,
}

impl Location {
    /// Compute a range suitable for lookup in pack data using the [`entry_slice()`][crate::data::File::entry_slice()] method.
    pub fn entry_range(&self, pack_offset: u64) -> crate::data::EntryRange {
        pack_offset..pack_offset + self.entry_size as u64
    }
}

///
pub mod init;

mod find;
///
pub mod write;

mod verify {
    use std::sync::{atomic::AtomicBool, Arc};

    use crate::Bundle;
    use git_features::progress::Progress;

    impl Bundle {
        /// Similar to [`crate::index::File::verify_integrity()`] but more convenient to call as the presence of the
        /// pack file is a given.
        pub fn verify_integrity<C, P>(
            &self,
            verify_mode: crate::index::verify::Mode,
            traversal: crate::index::traverse::Algorithm,
            make_pack_lookup_cache: impl Fn() -> C + Send + Sync,
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
