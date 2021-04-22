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
    #[error("Object id {oid} wasn't found in object database")]
    NotFound { oid: ObjectId },
}

/// The way input objects are handled
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectExpansion {
    /// Don't do anything with the input objects except for transforming them into pack entries
    AsIs,
}

impl Default for ObjectExpansion {
    fn default() -> Self {
        ObjectExpansion::AsIs
    }
}

/// Configuration options for the pack generation functions provied in [this module][crate::pack::data::encode].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Options {
    /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
    pub thread_limit: Option<usize>,
    /// The amount of objects per chunk or unit of work to be sent to threads for processing
    /// TODO: could this become the window size?
    chunk_size: usize,
    /// The pack data version to produce
    pub version: crate::pack::data::Version,
    /// The way input objects are handled
    pub input_object_expansion: ObjectExpansion,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            thread_limit: None,
            chunk_size: 10,
            version: Default::default(),
            input_object_expansion: Default::default(),
        }
    }
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
/// * **does not yet support thin packs** as we don't have a way to determine which objects are supposed to be thin.
/// * ~~currently there is no way to easily write the pack index, even though the state here is uniquely positioned to do
///   so with minimal overhead (especially compared to `gixp index-from-pack`)~~ Probably works now by chaining Iterators
///  or keeping enough state to write a pack and then generate an index with recorded data.
///
pub fn entries<Locate, Iter, Oid, Cache>(
    db: Locate,
    make_cache: impl Fn() -> Cache + Send + Clone + Sync + 'static,
    objects: Iter,
    _progress: impl Progress,
    Options {
        version,
        thread_limit,
        input_object_expansion,
        chunk_size,
    }: Options,
) -> impl Iterator<Item = Result<Vec<Entry>, Error<Locate::Error>>>
where
    Locate: crate::Locate + Clone + Send + Sync + 'static,
    <Locate as crate::Locate>::Error: Send,
    Iter: Iterator<Item = Oid> + Send + 'static,
    Oid: AsRef<oid> + Send + 'static,
    Cache: pack::cache::DecodeEntry,
{
    let lower_bound = objects.size_hint().0;
    let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(
        chunk_size,
        if lower_bound == 0 { None } else { Some(lower_bound) },
        thread_limit,
        None,
    );
    let chunks = util::Chunks {
        iter: objects,
        size: chunk_size,
    };

    parallel::reduce::Stepwise::new(
        chunks,
        thread_limit,
        move |_n| {
            (
                Vec::new(),   // object locate buffer
                make_cache(), // cache to speed up pack operations
            )
        },
        move |oids: Vec<Oid>, (buf, cache)| {
            use ObjectExpansion::*;
            let out = Vec::new();
            match input_object_expansion {
                AsIs => {
                    for id in oids.into_iter() {
                        let obj = db.locate(id.as_ref(), buf, cache)?.ok_or_else(|| Error::NotFound {
                            oid: id.as_ref().to_owned(),
                        })?;
                        match db.pack_entry(&obj) {
                            Some(entry) if entry.version == version => {
                                let _entry_data = pack::data::header::Entry::from_bytes(entry.data, 0);
                                todo!("pack to pack copy")
                            }
                            _ => {
                                todo!("encode pack entry from object data")
                            }
                        }
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(out)
        },
        parallel::reduce::IdentityWithResult::default(),
    )
}

mod util {
    pub struct Chunks<I> {
        pub size: usize,
        pub iter: I,
    }
    impl<I, Item> Iterator for Chunks<I>
    where
        I: Iterator<Item = Item>,
    {
        type Item = Vec<Item>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut res = Vec::with_capacity(self.size);
            let mut items_left = self.size;
            while let Some(item) = self.iter.next() {
                res.push(item);
                items_left -= 1;
                if items_left == 0 {
                    break;
                }
            }
            if res.is_empty() {
                None
            } else {
                Some(res)
            }
        }
    }
}
