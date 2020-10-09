use crate::pack;
use std::collections::BTreeMap;

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
    pub num_commits: u32,
    pub num_trees: u32,
    pub num_tags: u32,
    pub num_blobs: u32,
}

impl Default for Outcome {
    fn default() -> Self {
        Outcome {
            average: pack::data::decode::Outcome::default_from_kind(git_object::Kind::Tree),
            objects_per_chain_length: Default::default(),
            total_compressed_entries_size: 0,
            total_decompressed_entries_size: 0,
            total_object_size: 0,
            pack_size: 0,
            num_blobs: 0,
            num_commits: 0,
            num_trees: 0,
            num_tags: 0,
        }
    }
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
        matches!(self, SafetyCheck::All)
    }
    pub fn object_checksum(&self) -> bool {
        matches!(self, SafetyCheck::All | SafetyCheck::SkipFileChecksumVerification)
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
