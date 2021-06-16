use crate::index;
use git_features::{
    parallel,
    progress::{self, Progress},
};
use std::sync::{atomic::AtomicBool, Arc};

///
mod indexed;
mod reduce;
///
pub mod with_lookup;
pub(crate) use reduce::Reducer;

mod error;
pub use error::Error;

mod types;
pub use types::{Algorithm, Outcome, SafetyCheck};

mod options {
    use crate::index::traverse::{Algorithm, SafetyCheck};
    use std::sync::{atomic::AtomicBool, Arc};

    /// Traversal options for [`traverse()`][crate::index::File::traverse()]
    #[derive(Debug, Clone)]
    pub struct Options {
        /// The algorithm to employ.
        pub algorithm: Algorithm,
        /// If `Some`, only use the given amount of threads. Otherwise, the amount of threads to use will be selected based on
        /// the amount of available logical cores.
        pub thread_limit: Option<usize>,
        /// The kinds of safety checks to perform.
        pub check: SafetyCheck,
        /// A flag to indicate whether the algorithm should be interrupted. Will be checked occasionally allow stopping a running
        /// computation.
        pub should_interrupt: Arc<AtomicBool>,
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                algorithm: Algorithm::Lookup,
                thread_limit: Default::default(),
                check: Default::default(),
                should_interrupt: Default::default(),
            }
        }
    }
}
pub use options::Options;

/// Traversal of pack data files using an index file
impl index::File {
    /// Iterate through all _decoded objects_ in the given `pack` and handle them with a `Processor`.
    /// The return value is (pack-checksum, [`Outcome`], `progress`), thus the pack traversal will always verify
    /// the whole packs checksum to assure it was correct. In case of bit-rod, the operation will abort early without
    /// verifying all objects using the [interrupt mechanism][git_features::interrupt] mechanism.
    ///
    /// # Algorithms
    ///
    /// Using the [`Options::algorithm`] field one can chose between two algorithms providing different tradeoffs. Both invoke
    /// `new_processor()` to create functions receiving decoded objects, their object kind, index entry and a progress instance to provide
    /// progress information.
    ///
    /// * [`Algorithm::DeltaTreeLookup`] builds an index to avoid any unnecessary computation while resolving objects, avoiding
    ///   the need for a cache entirely, rendering `new_cache()` unused.
    ///   One could also call [`traverse_with_index()`][index::File::traverse_with_index()] directly.
    /// * [`Algorithm::Lookup`] uses a cache created by `new_cache()` to avoid having to re-compute all bases of a delta-chain while
    ///   decoding objects.
    ///   One could also call [`traverse_with_lookup()`][index::File::traverse_with_lookup()] directly.
    ///
    /// Use [`thread_limit`][Options::thread_limit] to further control parallelism and [`check`][SafetyCheck] to define how much the passed
    /// objects shall be verified beforehand.
    pub fn traverse<P, C, Processor, E>(
        &self,
        pack: &crate::data::File,
        progress: Option<P>,
        new_processor: impl Fn() -> Processor + Send + Sync,
        new_cache: impl Fn() -> C + Send + Sync,
        Options {
            algorithm,
            thread_limit,
            check,
            should_interrupt,
        }: Options,
    ) -> Result<(git_hash::ObjectId, Outcome, Option<P>), Error<E>>
    where
        P: Progress,
        C: crate::cache::DecodeEntry,
        E: std::error::Error + Send + Sync + 'static,
        Processor: FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &mut progress::DoOrDiscard<<<P as Progress>::SubProgress as Progress>::SubProgress>,
        ) -> Result<(), E>,
    {
        let progress = progress::DoOrDiscard::from(progress);
        match algorithm {
            Algorithm::Lookup => self.traverse_with_lookup(
                new_processor,
                new_cache,
                progress,
                pack,
                with_lookup::Options {
                    thread_limit,
                    check,
                    should_interrupt,
                },
            ),
            Algorithm::DeltaTreeLookup => {
                self.traverse_with_index(check, thread_limit, new_processor, progress, pack, should_interrupt)
            }
        }
        .map(|(a, b, p)| (a, b, p.into_inner()))
    }

    pub(crate) fn possibly_verify<E>(
        &self,
        pack: &crate::data::File,
        check: SafetyCheck,
        pack_progress: impl Progress,
        index_progress: impl Progress,
        should_interrupt: Arc<AtomicBool>,
    ) -> Result<git_hash::ObjectId, Error<E>>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Ok(if check.file_checksum() {
            if self.pack_checksum() != pack.checksum() {
                return Err(Error::PackMismatch {
                    actual: pack.checksum(),
                    expected: self.pack_checksum(),
                });
            }
            let (pack_res, id) = parallel::join(
                {
                    let should_interrupt = Arc::clone(&should_interrupt);
                    move || pack.verify_checksum(pack_progress, &should_interrupt)
                },
                move || self.verify_checksum(index_progress, &should_interrupt),
            );
            pack_res?;
            id?
        } else {
            self.index_checksum()
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn decode_and_process_entry<C, P, E>(
        &self,
        check: SafetyCheck,
        pack: &crate::data::File,
        cache: &mut C,
        buf: &mut Vec<u8>,
        progress: &mut P,
        header_buf: &mut [u8; 64],
        index_entry: &crate::index::Entry,
        processor: &mut impl FnMut(git_object::Kind, &[u8], &index::Entry, &mut P) -> Result<(), E>,
    ) -> Result<crate::data::decode_entry::Outcome, Error<E>>
    where
        C: crate::cache::DecodeEntry,
        P: Progress,
        E: std::error::Error + Send + Sync + 'static,
    {
        let pack_entry = pack.entry(index_entry.pack_offset);
        let pack_entry_data_offset = pack_entry.data_offset;
        let entry_stats = pack
            .decode_entry(
                pack_entry,
                buf,
                |id, _| {
                    self.lookup(id)
                        .map(|index| crate::data::ResolvedBase::InPack(pack.entry(self.pack_offset_at_index(index))))
                },
                cache,
            )
            .map_err(|e| Error::PackDecode {
                source: e,
                id: index_entry.oid,
                offset: index_entry.pack_offset,
            })?;
        let object_kind = entry_stats.kind;
        let header_size = (pack_entry_data_offset - index_entry.pack_offset) as usize;
        let entry_len = header_size + entry_stats.compressed_size;

        process_entry(
            check,
            object_kind,
            &buf,
            progress,
            header_buf,
            index_entry,
            || pack.entry_crc32(index_entry.pack_offset, entry_len),
            processor,
        )?;
        Ok(entry_stats)
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_entry<P, E>(
    check: SafetyCheck,
    object_kind: git_object::Kind,
    decompressed: &[u8],
    progress: &mut P,
    header_buf: &mut [u8; 64],
    index_entry: &crate::index::Entry,
    pack_entry_crc32: impl FnOnce() -> u32,
    processor: &mut impl FnMut(git_object::Kind, &[u8], &index::Entry, &mut P) -> Result<(), E>,
) -> Result<(), Error<E>>
where
    P: Progress,
    E: std::error::Error + Send + Sync + 'static,
{
    if check.object_checksum() {
        let header_size =
            crate::loose::object::header::encode(object_kind, decompressed.len() as u64, &mut header_buf[..])
                .expect("header buffer to be big enough");
        let mut hasher = git_features::hash::Sha1::default();
        hasher.update(&header_buf[..header_size]);
        hasher.update(decompressed);

        let actual_oid = git_hash::ObjectId::new_sha1(hasher.digest());
        if actual_oid != index_entry.oid {
            return Err(Error::PackObjectMismatch {
                actual: actual_oid,
                expected: index_entry.oid,
                offset: index_entry.pack_offset,
                kind: object_kind,
            });
        }
        if let Some(desired_crc32) = index_entry.crc32 {
            let actual_crc32 = pack_entry_crc32();
            if actual_crc32 != desired_crc32 {
                return Err(Error::Crc32Mismatch {
                    actual: actual_crc32,
                    expected: desired_crc32,
                    offset: index_entry.pack_offset,
                    kind: object_kind,
                });
            }
        }
    }
    processor(object_kind, decompressed, &index_entry, progress).map_err(Error::Processor)
}
