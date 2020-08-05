use crate::pack::{self, index, index::util::TimeThroughput};
use git_features::{
    parallel,
    progress::{self, Progress},
};
use git_object::owned;
use quick_error::quick_error;
use std::collections::BTreeMap;

mod indexed;
mod lookup;
mod reduce;
pub(crate) use reduce::Reducer;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Processor(err: Box<dyn std::error::Error + Send + Sync>) {
            display("One of the traversal processors failed")
            source(&**err)
            from()
        }
        Verify(err: index::verify::Error) {
            display("Index file, pack file or object verification failed")
            source(err)
            from()
        }
        Graph(err: pack::graph::Error) {
            display("The pack delta graph could not be built")
            from()
            source(err)
        }
        Tree(err: pack::tree::from_offsets::Error) {
            display("The pack delta tree index could not be built")
            from()
            source(err)
        }
        PackChecksum(err: pack::data::verify::Error) {
            display("The pack of this index file failed to verify its checksums")
            from()
            source(err)
        }
        PackDecode(err: pack::data::decode::Error, id: owned::Id, offset: u64) {
            display("Object {} at offset {} could not be decoded", id, offset)
            source(err)
        }
        PackMismatch { expected: owned::Id, actual: owned::Id } {
            display("The packfiles checksum didn't match the index file checksum: expected {}, got {}", expected, actual)
        }
        PackObjectMismatch { expected: owned::Id, actual: owned::Id, offset: u64, kind: git_object::Kind} {
            display("The SHA1 of {} object at offset {} didn't match the checksum in the index file: expected {}, got {}", kind, offset, expected, actual)
        }
        Crc32Mismatch { expected: u32, actual: u32, offset: u64, kind: git_object::Kind} {
            display("The CRC32 of {} object at offset {} didn't match the checksum in the index file: expected {}, got {}", kind, offset, expected, actual)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub average: pack::data::decode::Outcome,
    pub objects_per_chain_length: BTreeMap<u32, u32>,
    /// The amount of bytes in all compressed streams, one per entry
    pub total_compressed_entries_size: u64,
    /// The amount of bytes in all decompressed streams, one per entry
    pub total_decompressed_entries_size: u64,
    /// The amount of bytes occupied by all undeltified, decompressed objects
    pub total_object_size: u64,
    /// The amount of bytes occupied by the pack itself, in bytes
    pub pack_size: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum SafetyCheck {
    /// Don't verify the validity of the checksums stored in the index and pack file
    SkipFileChecksumVerification,

    /// All of the above, and also don't perform any object checksum verification
    SkipFileAndObjectChecksumVerification,

    /// All of the above, and only log object decode errors.
    ///
    /// Useful if there is a damaged pack and you would like to traverse as many objects as possible.
    SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError,

    /// Perform all available safety checks before operating on the pack and
    /// abort if any of them fails
    All,
}

impl SafetyCheck {
    pub fn file_checksum(&self) -> bool {
        match self {
            SafetyCheck::All => true,
            _ => false,
        }
    }
    pub fn object_checksum(&self) -> bool {
        match self {
            SafetyCheck::All | SafetyCheck::SkipFileChecksumVerification => true,
            _ => false,
        }
    }
    pub fn fatal_decode_error(&self) -> bool {
        match self {
            SafetyCheck::All
            | SafetyCheck::SkipFileChecksumVerification
            | SafetyCheck::SkipFileAndObjectChecksumVerification => true,
            SafetyCheck::SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError => false,
        }
    }
}

impl Default for SafetyCheck {
    fn default() -> Self {
        SafetyCheck::All
    }
}

/// The way we verify the pack
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Algorithm {
    /// Build an index to allow decoding each delta and base exactly once, saving a lot of computational
    /// resource at the expense of resident memory, as we will use an additional `DeltaTree` to accelerate
    /// delta chain resolution.
    DeltaTreeLookup,
    /// We lookup each object similarly to what would happen during normal repository use.
    /// Uses more compute resources as it will resolve delta chains from back to front, but start right away
    /// without indexing or investing any memory in indices.
    ///
    /// This option may be well suited for big packs in memory-starved system that support memory mapping.
    Lookup,
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::DeltaTreeLookup
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Options {
    pub algorithm: Algorithm,
    pub thread_limit: Option<usize>,
    pub check: SafetyCheck,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::Lookup,
            thread_limit: Default::default(),
            check: Default::default(),
        }
    }
}

/// Verify and validate the content of the index file
impl index::File {
    pub fn traverse<P, C, Processor>(
        &self,
        pack: &pack::data::File,
        Options {
            algorithm,
            thread_limit,
            check,
        }: Options,
        progress: Option<P>,
        new_processor: impl Fn() -> Processor + Send + Sync,
        make_cache: impl Fn() -> C + Send + Sync,
    ) -> Result<(owned::Id, Outcome, Option<P>), Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
        Processor: FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &pack::data::decode::Outcome,
            &mut progress::DoOrDiscard<<<P as Progress>::SubProgress as Progress>::SubProgress>,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
    {
        let mut root = progress::DoOrDiscard::from(progress);

        let id = if check.file_checksum() {
            let progress = root.add_child("Sha1 of index");
            let verify_self = move || self.verify_checksum(progress);

            if self.pack_checksum() != pack.checksum() {
                return Err(Error::PackMismatch {
                    actual: pack.checksum(),
                    expected: self.pack_checksum(),
                });
            }
            let mut progress = root.add_child("Sha1 of pack");
            let (pack_res, id) = parallel::join(
                move || {
                    let throughput = TimeThroughput::new(pack.data_len());
                    let res = pack.verify_checksum();
                    progress.done(throughput);
                    res
                },
                verify_self,
            );
            pack_res?;
            id?
        } else {
            self.index_checksum()
        };

        match algorithm {
            Algorithm::Lookup => self.traverse_with_lookup(check, thread_limit, new_processor, make_cache, root, pack),
            Algorithm::DeltaTreeLookup => {
                self.traverse_with_index_lookup(check, thread_limit, new_processor, root, pack)
            }
        }
        .map(|(stats, root)| (id, stats, root.into_inner()))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn process_entry_dispatch<C, P>(
        &self,
        check: SafetyCheck,
        pack: &pack::data::File,
        cache: &mut C,
        buf: &mut Vec<u8>,
        progress: &mut P,
        header_buf: &mut [u8; 64],
        index_entry: &pack::index::Entry,
        processor: &mut impl FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &pack::data::decode::Outcome,
            &mut P,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
    ) -> Result<pack::data::decode::Outcome, Error>
    where
        C: pack::cache::DecodeEntry,
        P: Progress,
    {
        let pack_entry = pack.entry(index_entry.pack_offset);
        let pack_entry_data_offset = pack_entry.data_offset;
        let entry_stats = pack
            .decode_entry(
                pack_entry,
                buf,
                |id, _| {
                    self.lookup(id).map(|index| {
                        pack::data::decode::ResolvedBase::InPack(pack.entry(self.pack_offset_at_index(index)))
                    })
                },
                cache,
            )
            .map_err(|e| Error::PackDecode(e, index_entry.oid, index_entry.pack_offset))?;
        let object_kind = entry_stats.kind;

        if check.object_checksum() {
            let header_size = crate::loose::object::header::encode(object_kind, buf.len() as u64, &mut header_buf[..])
                .expect("header buffer to be big enough");
            let mut hasher = git_features::hash::Sha1::default();
            hasher.update(&header_buf[..header_size]);
            hasher.update(buf.as_slice());

            let actual_oid = owned::Id::new_sha1(hasher.digest());
            if actual_oid != index_entry.oid {
                return Err(Error::PackObjectMismatch {
                    actual: actual_oid,
                    expected: index_entry.oid,
                    offset: index_entry.pack_offset,
                    kind: object_kind,
                });
            }
            if let Some(desired_crc32) = index_entry.crc32 {
                let header_size = (pack_entry_data_offset - index_entry.pack_offset) as usize;
                let actual_crc32 = pack.entry_crc32(index_entry.pack_offset, header_size + entry_stats.compressed_size);
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
        processor(object_kind, buf.as_slice(), &index_entry, &entry_stats, progress)?;
        Ok(entry_stats)
    }
}
