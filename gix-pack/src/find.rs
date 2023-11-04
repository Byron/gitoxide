/// An Entry in a pack providing access to its data.
///
/// Its commonly retrieved by reading from a pack index file followed by a read from a pack data file.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub struct Entry {
    /// The pack-data encoded bytes of the pack data entry as present in the pack file, including the header followed by compressed data.
    pub data: Vec<u8>,
    /// The version of the pack file containing `data`
    pub version: crate::data::Version,
}
