use crate::{pack, pack::index};
use git_features::{
    parallel,
    progress::{self, Progress},
};
use git_object::{borrowed, bstr::BString, owned};
use quick_error::quick_error;
use std::time::Instant;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Processor(err: Box<dyn std::error::Error + Send>) {
            source(&**err)
            from()
        }
        Verify(err: index::verify::Error) {
            source(err)
            from()
        }
        Io(err: std::io::Error, path: std::path::PathBuf, msg: &'static str) {
            display("Failed to {} at path '{}'", msg, path.display())
            source(err)
        }
        Graph(err: pack::graph::Error) {
            from()
            source(err)
        }
        Mismatch { expected: owned::Id, actual: owned::Id } {
            display("index checksum mismatch: expected {}, got {}", expected, actual)
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
        ObjectDecode(err: borrowed::Error, kind: git_object::Kind, oid: owned::Id) {
            display("{} object {} could not be decoded", kind, oid)
            source(err)
        }
        ObjectEncodeMismatch(kind: git_object::Kind, oid: owned::Id, expected: BString, actual: BString) {
            display("{} object {} wasn't re-encoded without change, wanted\n{}\n\nGOT\n\n{}", kind, oid, expected, actual)
        }
        ObjectEncode(err: std::io::Error) {
            from()
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

struct TimeThroughput {
    then: Instant,
    byte_size: usize,
}

impl TimeThroughput {
    pub fn new(byte_size: usize) -> TimeThroughput {
        TimeThroughput {
            then: Instant::now(),
            byte_size,
        }
    }
}

impl Into<String> for TimeThroughput {
    fn into(self) -> String {
        let time_taken = std::time::Instant::now().duration_since(self.then).as_secs_f32();
        format!(
            "finished in {:.2}s at {}/s",
            time_taken,
            bytesize::ByteSize((self.byte_size as f32 / time_taken) as u64)
        )
    }
}

/// The way we verify the pack
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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

mod indexed;
mod lookup;
mod reduce;
pub(crate) use reduce::Reducer;

/// Verify and validate the content of the index file
impl index::File {
    pub fn traverse_index<P, C, Processor>(
        &self,
        pack: &pack::data::File,
        algorithm: Algorithm,
        thread_limit: Option<usize>,
        progress: Option<P>,
        new_processor: impl Fn() -> Processor + Send + Sync,
        make_cache: impl Fn() -> C + Send + Sync,
    ) -> Result<(owned::Id, index::verify::Outcome), Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
        Processor: FnMut() -> Result<(), Box<dyn std::error::Error + Send>>,
    {
        let mut root = progress::DoOrDiscard::from(progress);

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
        let id = id?;

        match algorithm {
            Algorithm::Lookup => self.traverse_with_lookup(thread_limit, new_processor, make_cache, root, pack),
            Algorithm::DeltaTreeLookup => self.traverse_with_index_lookup(thread_limit, new_processor, root, pack),
        }
        .map(|stats| (id, stats))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn process_entry_dispatch<C>(
        &self,
        pack: &pack::data::File,
        cache: &mut C,
        buf: &mut Vec<u8>,
        _progress: &mut impl Progress,
        header_buf: &mut [u8; 64],
        index_entry: &pack::index::Entry,
        processor: &mut impl FnMut() -> Result<(), Box<dyn std::error::Error + Send>>,
    ) -> Result<pack::data::decode::Outcome, Error>
    where
        C: pack::cache::DecodeEntry,
    {
        let pack_entry = pack.entry(index_entry.pack_offset);
        let pack_entry_data_offset = pack_entry.data_offset;
        let entry_stats = pack
            .decode_entry(
                pack_entry,
                buf,
                |id, _| {
                    self.lookup_index(id).map(|index| {
                        pack::data::decode::ResolvedBase::InPack(pack.entry(self.pack_offset_at_index(index)))
                    })
                },
                cache,
            )
            .map_err(|e| Error::PackDecode(e, index_entry.oid, index_entry.pack_offset))?;
        let object_kind = entry_stats.kind;
        let consumed_input = entry_stats.compressed_size;

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
            let actual_crc32 = pack.entry_crc32(index_entry.pack_offset, header_size + consumed_input);
            if actual_crc32 != desired_crc32 {
                return Err(Error::Crc32Mismatch {
                    actual: actual_crc32,
                    expected: desired_crc32,
                    offset: index_entry.pack_offset,
                    kind: object_kind,
                });
            }
        }
        processor()?;
        Ok(entry_stats)
    }
}
