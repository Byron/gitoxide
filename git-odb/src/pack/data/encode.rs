use crate::pack;
use git_features::{parallel, progress::Progress};
use git_hash::{oid, ObjectId};

/// The error returned the pack generation functions in [this module][crate::pack::data::encode].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error<LocateErr>
where
    LocateErr: std::error::Error + 'static,
{
    #[error(transparent)]
    Locate(#[from] LocateErr),
}

/// Configuration options for the pack generation functions provied in [this module][crate::pack::data::encode].
#[derive(Default, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
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
/// * Abstract over object databases and how input is provided.
///
/// ### Disadvantages
///
/// * No support yet for pack-to-pack copies, but that can be added if `data::Objects` or whatever `locate()` returns
///   keeps track of the owning pack. This should be quite trivial to do with the added cost of keeping track of packs.
/// * **does not yet support thin packs** as we don't have a way to determine which objects are supposed to be thin.
/// * ~~currently there is no way to easily write the pack index, even though the state here is uniquely positioned to do
///   so with minimal overhead (especially compared to `gixp index-from-pack`)~~ Probably works now by chaining Iterators
///  or keeping enough state to write a pack and then generate an index with recorded data.
///
pub fn entries<Locate, Iter, Oid>(
    db: Locate,
    objects: Iter,
    _progress: impl Progress,
    Options { version, thread_limit }: Options,
) -> impl Iterator<Item = Result<Vec<Entry>, Error<Locate::Error>>>
where
    Locate: crate::Locate + Clone + Send + Sync + 'static,
    <Locate as crate::Locate>::Error: Send,
    Iter: Iterator<Item = Oid> + Send + 'static,
    Oid: AsRef<oid> + Send + 'static,
{
    use git_features::parallel::reduce;

    reduce::Stepwise::new(
        objects,
        thread_limit,
        |_n| (Vec::new(), pack::cache::Noop),
        move |oid, (buf, cache)| {
            let _obj = db.locate(oid.as_ref(), buf, cache)?;
            let _ = version; // currently unused
            todo!("entry generation");
            #[allow(unreachable_code)]
            Ok(Vec::new())
        },
        parallel::reduce::IdentityWithResult::default(),
    )
}
