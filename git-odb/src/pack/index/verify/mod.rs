use crate::{pack, pack::data::decode, pack::index};
use git_features::{
    parallel,
    progress::{self, Progress},
};
use git_object::{borrowed, bstr::BString, owned, SHA1_SIZE};
use quick_error::quick_error;
use std::{collections::BTreeMap, time::Instant};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
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

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub average: decode::Outcome,
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

/// Various ways in which a pack and index can be verified
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Mode {
    /// Validate SHA1 and CRC32
    Sha1CRC32,
    /// Validate SHA1 and CRC32, and decode each non-Blob object
    Sha1CRC32Decode,
    /// Validate SHA1 and CRC32, and decode and encode each non-Blob object
    Sha1CRC32DecodeEncode,
}

/// The way we verify the pack
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Algorithm {
    /// We lookup each object similarly to what would happen during normal repository use.
    /// Uses more compute resources as it will resolve delta chains from back to front, potentially
    Lookup,
    /// Build an index to allow decoding each delta and base exactly once, saving a lot of computational
    /// resource at the expense of resident memory, as we will use an additional DeltaTree to make that happen.
    DeltaTreeLookup,
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Lookup
    }
}

mod indexed;
mod lookup;

/// Verify and validate the content of the index file
impl index::File {
    pub fn checksum_of_index(&self) -> owned::Id {
        owned::Id::from_20_bytes(&self.data[self.data.len() - SHA1_SIZE..])
    }

    pub fn checksum_of_pack(&self) -> owned::Id {
        let from = self.data.len() - SHA1_SIZE * 2;
        owned::Id::from_20_bytes(&self.data[from..from + SHA1_SIZE])
    }

    /// If `pack` is provided, it is expected (and validated to be) the pack belonging to this index.
    /// It will be used to validate internal integrity of the pack before checking each objects integrity
    /// is indeed as advertised via its SHA1 as stored in this index, as well as the CRC32 hash.
    /// redoing a lot of work across multiple objects.
    pub fn verify_checksum_of_index<P, C>(
        &self,
        pack: Option<(&pack::data::File, Mode, Algorithm)>,
        thread_limit: Option<usize>,
        progress: Option<P>,
        make_cache: impl Fn() -> C + Send + Sync,
    ) -> Result<(owned::Id, Option<Outcome>), Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
    {
        let mut root = progress::DoOrDiscard::from(progress);
        let mut progress = root.add_child("Sha1 of index");

        let mut verify_self = move || {
            let throughput = TimeThroughput::new(self.data.len());
            let mut hasher = git_features::hash::Sha1::default();
            hasher.update(&self.data[..self.data.len() - SHA1_SIZE]);
            let actual = owned::Id::new_sha1(hasher.digest());
            progress.done(throughput);

            let expected = self.checksum_of_index();
            if actual == expected {
                Ok(actual)
            } else {
                Err(Error::Mismatch { actual, expected })
            }
        };
        match pack {
            None => verify_self().map(|id| (id, None)),
            Some((pack, mode, algorithm)) => {
                if self.checksum_of_pack() != pack.checksum() {
                    return Err(Error::PackMismatch {
                        actual: pack.checksum(),
                        expected: self.checksum_of_pack(),
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
                    Algorithm::Lookup => self.inner_verify_with_lookup(thread_limit, mode, make_cache, root, pack),
                    Algorithm::DeltaTreeLookup => {
                        self.inner_verify_with_indexed_lookup(thread_limit, mode, make_cache, root, pack)
                    }
                }
                .map(|stats| (id, Some(stats)))
            }
        }
    }
}
