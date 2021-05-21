use crate::{find, pack, pack::data::output, FindExt};
use git_features::{parallel, progress::Progress};
use std::sync::Arc;

/// Given a known list of object `counts`, calculate entries ready to be put into a data pack.
///
/// This allows objects to be written quite soon without having to wait for the entire pack to be built in memory.
/// A chunk of objects is held in memory and compressed using DEFLATE, and serve the output of this iterator.
/// That way slow writers will naturally apply back pressure, and communicate to the implementation that more time can be
/// spent compressing objects.
///
/// * `counts`
///   * A list of previously counted objects to add to the pack. Duplication checks are not performed, no object is expected to be duplicated.
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
pub fn objects_to_entries_iter<Find, Cache>(
    counts: Vec<output::Count>,
    db: Find,
    make_cache: impl Fn() -> Cache + Send + Clone + Sync + 'static,
    _progress: impl Progress,
    Options {
        version,
        thread_limit,
        chunk_size,
    }: Options,
) -> impl Iterator<Item = Result<Vec<output::Entry>, Error<find::existing::Error<Find::Error>>>>
       + parallel::reduce::Finalize<
    Reduce = parallel::reduce::IdentityWithResult<Vec<output::Entry>, Error<find::existing::Error<Find::Error>>>,
>
where
    Find: crate::Find + Clone + Send + Sync + 'static,
    <Find as crate::Find>::Error: Send,
    Cache: pack::cache::DecodeEntry,
{
    assert!(
        matches!(version, pack::data::Version::V2),
        "currently we can only write version 2"
    );
    let counts = Arc::new(counts);
    let (chunk_size, thread_limit, _) =
        parallel::optimize_chunk_size_and_thread_limit(chunk_size, Some(counts.len()), thread_limit, None);
    let chunks = util::Chunks::new(chunk_size, counts.len());

    parallel::reduce::Stepwise::new(
        chunks,
        thread_limit,
        move |_n| {
            (
                Vec::new(),   // object data buffer
                make_cache(), // cache to speed up pack operations
            )
        },
        {
            let counts = Arc::clone(&counts);
            move |chunk: std::ops::Range<usize>, (buf, cache)| {
                let mut out = Vec::new();
                let chunk = &counts[chunk];
                for count in chunk.into_iter() {
                    out.push(
                        match count.entry_pack_location.as_ref().and_then(|l| db.pack_entry(l)) {
                            Some(pack_entry) => match output::Entry::from_pack_entry(pack_entry, count, version) {
                                Some(entry) => entry,
                                None => {
                                    let obj = db.find_existing(count.id, buf, cache).map_err(Error::FindExisting)?;
                                    output::Entry::from_data(count, &obj)
                                }
                            },
                            None => {
                                let obj = db.find_existing(count.id, buf, cache).map_err(Error::FindExisting)?;
                                output::Entry::from_data(count, &obj)
                            }
                        }?,
                    );
                }
                Ok(out)
            }
        },
        parallel::reduce::IdentityWithResult::default(),
    )
}

mod util {
    pub struct Chunks {
        cursor: usize,
        size: usize,
        len: usize,
    }

    impl Chunks {
        pub fn new(size: usize, total: usize) -> Self {
            Chunks {
                cursor: 0,
                size,
                len: total,
            }
        }
    }

    impl Iterator for Chunks {
        type Item = std::ops::Range<usize>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cursor >= self.len {
                None
            } else {
                let range = self.cursor..(self.cursor + self.size).min(self.len);
                self.cursor += self.size;
                Some(range)
            }
        }
    }
}

mod types {
    use crate::pack::data::output::entry;

    /// Configuration options for the pack generation functions provied in [this module][crate::pack::data::output].
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Options {
        /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
        pub thread_limit: Option<usize>,
        /// The amount of objects per chunk or unit of work to be sent to threads for processing
        /// TODO: could this become the window size?
        pub chunk_size: usize,
        /// The pack data version to produce
        pub version: crate::pack::data::Version,
    }

    impl Default for Options {
        fn default() -> Self {
            Options {
                thread_limit: None,
                chunk_size: 10,
                version: Default::default(),
            }
        }
    }

    /// The error returned by the pack generation function [`to_entry_iter()`][crate::pack::data::output::objects_to_entries_iter()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<FindErr>
    where
        FindErr: std::error::Error + 'static,
    {
        #[error(transparent)]
        FindExisting(FindErr),
        #[error(transparent)]
        NewEntry(#[from] entry::Error),
    }
}
pub use types::{Error, Options};
