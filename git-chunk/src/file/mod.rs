///
pub mod decode;
///
pub mod index;

/// The offset to a chunk as seen relative to the beginning of the file containing it.
pub type Offset = u64;

/// A chunk file providing a table into the parent data.
pub struct Index {
    /// Validated chunks as defined by their index entries.
    ///
    /// Note that this list cannot be empty.
    pub chunks: Vec<index::Entry>,
}
