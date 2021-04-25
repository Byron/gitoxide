use crate::{pack, pack::data::output};
use git_features::{hash, parallel, progress::Progress};
use git_hash::oid;

/// The way input objects are handled
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Expansion {
    /// Don't do anything with the input objects except for transforming them into pack entries
    AsIs,
}

impl Default for Expansion {
    fn default() -> Self {
        Expansion::AsIs
    }
}

/// Configuration options for the pack generation functions provied in [this module][crate::pack::data::output].
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
    pub input_object_expansion: Expansion,
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
pub fn to_entry_iter<Locate, Iter, Oid, Cache>(
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
) -> impl Iterator<Item = Result<Vec<output::Entry>, output::Error<Locate::Error>>>
       + parallel::reduce::Finalize<
    Reduce = parallel::reduce::IdentityWithResult<Vec<output::Entry>, output::Error<Locate::Error>>,
>
where
    Locate: crate::Locate + Clone + Send + Sync + 'static,
    <Locate as crate::Locate>::Error: Send,
    Iter: Iterator<Item = Oid> + Send + 'static,
    Oid: AsRef<oid> + Send + 'static,
    Cache: pack::cache::DecodeEntry,
{
    assert!(
        matches!(version, pack::data::Version::V2),
        "currently we can only write version 2"
    );
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
            use Expansion::*;
            let mut out = Vec::new();
            match input_object_expansion {
                AsIs => {
                    for id in oids.into_iter() {
                        let obj = db
                            .locate(id.as_ref(), buf, cache)?
                            .ok_or_else(|| output::Error::NotFound {
                                oid: id.as_ref().to_owned(),
                            })?;
                        out.push(match db.pack_entry(&obj) {
                            Some(entry) if entry.version == version => {
                                let pack_entry = pack::data::Entry::from_bytes(entry.data, 0);
                                if let Some(expected) = entry.crc32 {
                                    let actual = hash::crc32(entry.data);
                                    if actual != expected {
                                        return Err(output::Error::PackToPackCopyCrc32Mismatch { actual, expected });
                                    }
                                }
                                if pack_entry.header.is_base() {
                                    output::Entry {
                                        id: id.as_ref().to_owned(),
                                        object_kind: pack_entry.header.to_kind().expect("non-delta"),
                                        entry_kind: output::entry::Kind::Base,
                                        decompressed_size: obj.data.len(),
                                        compressed_data: entry.data[pack_entry.data_offset as usize..].to_owned(),
                                    }
                                } else {
                                    output::Entry::from_data(id.as_ref(), &obj).map_err(output::Error::NewEntry)?
                                }
                            }
                            _ => output::Entry::from_data(id.as_ref(), &obj).map_err(output::Error::NewEntry)?,
                        });
                    }
                }
            }
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
