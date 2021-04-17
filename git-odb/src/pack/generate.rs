use git_features::progress::Progress;
use git_hash::{oid, ObjectId};
use std::convert::TryInto;
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

/// Meta data of any object
pub struct ObjectHeader {
    /// The kind of object
    pub kind: git_object::Kind,
    /// The decompressed size of the objects raw data.
    pub size: u64,
}

/// An object that can represent no less than three different kinds of data all to avoid unnecessary copies or allocations.
///
/// * loose objects
/// * decompressed packed objects
/// * entries in packs
pub trait Object {
    /// Returns decompressed object data, or None if there is None.
    /// If that's the case, [`Object::read_all()`] is expected to deliver said data.
    fn data(&self) -> Option<(ObjectHeader, &[u8])> {
        None
    }

    /// Read all decompressed data into the given buffer, resizing it as needed.
    /// Returns None if this mode of operation is not supported.
    fn read_all(&mut self, buf: &mut Vec<u8>) -> Option<Result<ObjectHeader, std::io::Error>> {
        self.data().map(|(h, d)| {
            buf.resize(h.size.try_into().expect("size to be representable"), 0);
            buf.copy_from_slice(d);
            Ok(h)
        })
    }
}

/// Write all `objects` into `out` without attempting to apply any delta compression.
/// This allows objects to be written rather immediately.
/// Objects are held in memory and compressed using DEFLATE, with those in-flight chunks of compressed
/// objects being sent to the current thread for writing. No buffering of these objects is performed,
/// allowing for natural back-pressure in case of slow writers.
///
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
pub fn immediate<'a, Iter, Object>(
    _objects: Iter,
    _out: impl io::Write,
    _progress: impl Progress,
    _options: Options,
) -> Result<ObjectId, Error>
where
    Iter: ExactSizeIterator<Item = (&'a oid, Object)> + 'a,
    Object: AsMut<Object>,
{
    todo!()
}
