use std::collections::BTreeMap;

/// Statistics regarding object encountered during execution of the [`traverse()`][crate::index::File::traverse()] method.
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Statistics {
    /// The average over all decoded objects
    pub average: crate::data::decode::entry::Outcome,
    /// A mapping of the length of the chain to the amount of objects at that length.
    ///
    /// A length of 0 indicates full objects, and everything above that involves the given amount
    /// of delta objects.
    pub objects_per_chain_length: BTreeMap<u32, u32>,
    /// The amount of bytes in all compressed streams, one per entry
    pub total_compressed_entries_size: u64,
    /// The amount of bytes in all decompressed streams, one per entry
    pub total_decompressed_entries_size: u64,
    /// The amount of bytes occupied by all undeltified, decompressed objects
    pub total_object_size: u64,
    /// The amount of bytes occupied by the pack itself, in bytes
    pub pack_size: u64,
    /// The amount of objects encountered that where commits
    pub num_commits: u32,
    /// The amount of objects encountered that where trees
    pub num_trees: u32,
    /// The amount of objects encountered that where tags
    pub num_tags: u32,
    /// The amount of objects encountered that where blobs
    pub num_blobs: u32,
}

impl Default for Statistics {
    fn default() -> Self {
        Statistics {
            average: crate::data::decode::entry::Outcome::default_from_kind(git_object::Kind::Tree),
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

/// The ways to validate decoded objects before passing them to the processor.
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
    pub(crate) fn file_checksum(&self) -> bool {
        matches!(self, SafetyCheck::All)
    }
    pub(crate) fn object_checksum(&self) -> bool {
        matches!(self, SafetyCheck::All | SafetyCheck::SkipFileChecksumVerification)
    }
    pub(crate) fn fatal_decode_error(&self) -> bool {
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
