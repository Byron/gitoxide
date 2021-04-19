use git_features::progress::Progress;
use git_hash::{oid, ObjectId};
use std::convert::TryInto;

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
    /// Provide basic information about the object
    fn header(&self) -> ObjectHeader;

    /// Returns decompressed object data, or None if there is None.
    /// If that's the case, [`Object::read_all()`] is expected to deliver said data.
    fn data(&self) -> Option<&[u8]> {
        None
    }

    /// Read all decompressed data into the given buffer, resizing it as needed.
    /// Returns None if this mode of operation is not supported.
    fn read_all(&mut self, buf: &mut Vec<u8>) -> Option<Result<(), std::io::Error>> {
        self.data().map(|d| {
            let h = self.header();
            buf.resize(h.size.try_into().expect("size to be representable"), 0);
            buf.copy_from_slice(d);
            Ok(())
        })
    }

    /// Returns the packed entry if this object is indeed a base object allowing to copy data from pack to pack
    /// and avoiding a decompress/compress round-trip for some objects.
    fn packed_base_data(&self) -> Option<&[u8]> {
        None
    }
}

/// The kind of pack entry to be written
pub enum EntryKind {
    /// A complete base object
    Base,
    /// A delta against the object encountered `n` objects before (in this iteration)
    DeltaRef {
        /// Never 0, and 1 would mean the previous object acts as base object.
        nth_before: usize,
    },
    /// A delta against the given object as identified by its `ObjectId`.
    /// This is the case for thin packs only.
    /// Note that there is the option of the `ObjectId` being used to refer to an object within
    /// the same pack, but it's a discontinued practice which won't be encountered here.
    DeltaOid {
        /// The object serving as base for this delta
        id: ObjectId,
    },
}

/// An entry to be written to a file.
pub struct Entry {
    /// The hash of the object to write
    pub id: ObjectId,
    /// The kind of packed object
    pub object_kind: git_object::Kind,
    /// The kind of entry represented by `data`. It's used alongside with it to complete the pack entry
    /// at rest or in transit.
    pub entry_kind: EntryKind,
    /// The compressed data right behind the header
    pub data: Vec<u8>,
}

/// Write all `objects` into `out` without attempting to apply any delta compression.
/// This allows objects to be written rather immediately.
/// Objects are held in memory and compressed using DEFLATE, with those in-flight chunks of compressed
/// objects being sent to the current thread for writing. No buffering of these objects is performed,
/// allowing for natural back-pressure in case of slow writers.
///
/// * `objects`
///   * the fully expanded list of objects, no expansion will be performed here.
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
/// * Begins writing immediately and supports back-pressure.
///
/// ### Disadvantages
///
/// * **does not yet support thin packs** as we don't have a way to determine which objects are supposed to be thin.
/// * needs the traversal to have happened before, probably producing a `Vec<AsMut<Object>>` anyway. This implies
///   plenty of objects have been decompressed and parsed already, and will potentially be parsed twice.
///   * Live with the amount of objects being known in advance and get rid of the ExactIterator requirement OR
///     allow all iterations to have a 'quick' mode to pre-determine how many objects there will be in advance.
/// * ~~currently there is no way to easily write the pack index, even though the state here is uniquely positioned to do
///   so with minimal overhead (especially compared to `gixp index-from-pack`)~~ Probably works now by chaining Iterators
///  or keeping enough state to write a pack and then generate an index with recorded data.
///
pub fn entries<Iter, Object, Oid, O>(
    objects: Iter,
    _progress: impl Progress,
    Options { version, thread_limit }: Options,
) -> impl Iterator<Item = Result<Vec<Entry>, Error>>
where
    Iter: Iterator<Item = (Oid, Object)> + Send + 'static,
    Object: AsMut<O> + Send + 'static,
    O: self::Object,
    Oid: AsRef<oid> + Send + 'static,
{
    use git_features::parallel::{Reducer, SteppedReduce};

    struct Aggregator;
    impl Reducer for Aggregator {
        type Input = Vec<Entry>;
        type FeedProduce = Vec<Entry>;
        type Output = ();
        type Error = Error;

        fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
            Ok(item)
        }

        fn finalize(self) -> Result<Self::Output, Self::Error> {
            Ok(())
        }
    }

    SteppedReduce::new(
        objects,
        thread_limit,
        |_n| (),
        move |(_oid, _obj), _state| {
            let _ = version; // currently unused
            Vec::new()
        },
        Aggregator,
    )
}
