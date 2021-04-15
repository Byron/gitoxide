use crate::linked;
use git_features::progress::Progress;
use git_hash::{oid, ObjectId};
use std::io;

/// The error returned the pack generation functions in [this module][crate::pack::generate].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("TBD")]
    Tbd,
}

/// Configuration options for the pack generation functions provied in [this module][crate::pack::generate].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Options {
    /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
    pub thread_limit: Option<usize>,
    /// The pack data version to produce
    pub version: crate::pack::data::Version,
}

/// Write all `objects` into `out` without attempting to apply any delta compression.
/// This allows objects to be written rather immediately.
/// Objects are held in memory and compressed using DEFLATE, with those in-flight chunks of compressed
/// objects being sent to the current thread for writing. No buffering of these objects is performed,
/// allowing for natural back-pressure in case of slow writers.
///
/// * `odb`
///   * a way to lookup all provided `objects`.
/// * `objects`
///   * the fully expanded list of objects, no expansion will be performed here.
/// * `out`
///   * where to write to
/// * `progress`
///   * a way to obtain progress information
/// * `options`
///   * more configuration
///
/// _Returns_ the checksum of the pack
///
/// ## Discussion
///
/// ### Advantages
///
/// * Will always recompress and thus potentially allow for higher compression levels. It's probably a weak argument as
///   an option can also force recompression.
/// * Begins writing immediately and supports back-pressure.
///
/// ### Disadvantages
///
/// * cannot copy base objects directly from other packs (has to decompress first just to recompress).
/// * needs the traversal to have happened before, probably producing a `Vec<ObjectId>` anyway. This implies
///   plenty of objects have been decompressed and parsed already, and will now be parsed twice.
/// * currently there is no way to easily write the pack index, even though the state here is uniquely positioned to do
///   so with minimal overhead (especially compared to `gixp index-from-pack`).
///
pub fn immediate<Iter, Object>(
    _odb: &linked::Db,
    _objects: Iter,
    _out: impl io::Write,
    _progress: impl Progress,
    _options: Options,
) -> Result<ObjectId, Error>
where
    Iter: ExactSizeIterator<Item = Object>,
    Object: AsRef<oid>,
{
    todo!()
}
