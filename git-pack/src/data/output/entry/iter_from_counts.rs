use crate::{
    data::{output, output::ChunkId},
    find, FindExt,
};
use git_features::{parallel, progress::Progress};
use std::{cmp::Ordering, sync::Arc};

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
pub fn iter_from_counts<Find, Cache>(
    mut counts: Vec<output::Count>,
    db: Find,
    make_cache: impl Fn() -> Cache + Send + Clone + Sync + 'static,
    mut progress: impl Progress,
    Options {
        version,
        mode,
        allow_thin_pack,
        thread_limit,
        chunk_size,
    }: Options,
) -> impl Iterator<Item = Result<(ChunkId, Vec<output::Entry>), Error<find::existing::Error<Find::Error>>>>
       + parallel::reduce::Finalize<Reduce = reduce::Statistics<Error<find::existing::Error<Find::Error>>>>
where
    Find: crate::Find + Clone + Send + Sync + 'static,
    <Find as crate::Find>::Error: Send,
    Cache: crate::cache::DecodeEntry,
{
    assert!(
        matches!(version, crate::data::Version::V2),
        "currently we can only write version 2"
    );
    let counts_range_by_pack_id = match mode {
        Mode::PackCopyAndBaseObjects => {
            let mut progress = progress.add_child("sorting");
            progress.init(Some(counts.len()), git_features::progress::count("counts"));
            let start = std::time::Instant::now();

            counts.sort_by(|lhs, rhs| match (&lhs.entry_pack_location, &rhs.entry_pack_location) {
                (None, None) => Ordering::Equal,
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (Some(lhs), Some(rhs)) => lhs
                    .pack_id
                    .cmp(&rhs.pack_id)
                    .then(lhs.pack_offset.cmp(&rhs.pack_offset)),
            });

            let mut index: Vec<(u32, std::ops::Range<usize>)> = Vec::new();
            let mut pack_start = counts.partition_point(|e| e.entry_pack_location.is_none());
            let mut slice = &counts[pack_start..];
            while !slice.is_empty() {
                let current_pack_id = slice[0].entry_pack_location.as_ref().expect("packed object").pack_id;
                let pack_end = slice.partition_point(|e| {
                    e.entry_pack_location.as_ref().expect("packed object").pack_id == current_pack_id
                });
                index.push((current_pack_id, pack_start..pack_end));
                slice = &slice[pack_end..];
                pack_start = pack_end;
            }

            progress.set(counts.len());
            progress.show_throughput(start);

            index
        }
    };
    let counts = Arc::new(counts);
    let (chunk_size, thread_limit, _) =
        parallel::optimize_chunk_size_and_thread_limit(chunk_size, Some(counts.len()), thread_limit, None);
    let chunks = util::ChunkRanges::new(chunk_size, counts.len()).enumerate();
    let progress = Arc::new(parking_lot::Mutex::new(progress));

    parallel::reduce::Stepwise::new(
        chunks,
        thread_limit,
        {
            let progress = Arc::clone(&progress);
            move |n| {
                (
                    Vec::new(),   // object data buffer
                    make_cache(), // cache to speed up pack operations
                    progress.lock().add_child(format!("thread {}", n)),
                )
            }
        },
        {
            let counts = Arc::clone(&counts);
            move |(chunk_id, chunk): (ChunkId, std::ops::Range<usize>), (buf, cache, progress)| {
                let mut out = Vec::new();
                let chunk = &counts[chunk];
                let mut stats = Outcome::default();
                progress.init(Some(chunk.len()), git_features::progress::count("objects"));

                for count in chunk {
                    out.push(match count
                        .entry_pack_location
                        .as_ref()
                        .and_then(|l| db.entry_by_location(l).map(|pe| (l, pe)))
                    {
                        Some((location, pack_entry)) => {
                            let pack_range = counts_range_by_pack_id[counts_range_by_pack_id
                                .binary_search_by_key(&location.pack_id, |e| e.0)
                                .expect("pack-id always present")]
                            .1
                            .clone();
                            let base_index_offset = pack_range.start;
                            let counts_in_pack = &counts[pack_range];
                            match output::Entry::from_pack_entry(
                                pack_entry,
                                count,
                                counts_in_pack,
                                base_index_offset,
                                if allow_thin_pack {
                                    Some(|base_offset| todo!("lookup offset -> id"))
                                } else {
                                    None
                                },
                                version,
                            ) {
                                Some(entry) => {
                                    stats.objects_copied_from_pack += 1;
                                    entry
                                }
                                None => {
                                    let obj = db.find_existing(count.id, buf, cache).map_err(Error::FindExisting)?;
                                    stats.decoded_and_recompressed_objects += 1;
                                    output::Entry::from_data(count, &obj)
                                }
                            }
                        }
                        None => {
                            let obj = db.find_existing(count.id, buf, cache).map_err(Error::FindExisting)?;
                            stats.decoded_and_recompressed_objects += 1;
                            output::Entry::from_data(count, &obj)
                        }
                    }?);
                    progress.inc();
                }
                Ok((chunk_id, out, stats))
            }
        },
        reduce::Statistics::default(),
    )
}

mod util {
    pub struct ChunkRanges {
        cursor: usize,
        size: usize,
        len: usize,
    }

    impl ChunkRanges {
        pub fn new(size: usize, total: usize) -> Self {
            ChunkRanges {
                cursor: 0,
                size,
                len: total,
            }
        }
    }

    impl Iterator for ChunkRanges {
        type Item = std::ops::Range<usize>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cursor >= self.len {
                None
            } else {
                let upper = (self.cursor + self.size).min(self.len);
                let range = self.cursor..upper;
                self.cursor = upper;
                Some(range)
            }
        }
    }
}

mod reduce {
    use super::{ChunkId, Outcome};
    use crate::data::output;
    use git_features::parallel;
    use std::marker::PhantomData;

    pub struct Statistics<E> {
        total: Outcome,
        _err: PhantomData<E>,
    }

    impl<E> Default for Statistics<E> {
        fn default() -> Self {
            Statistics {
                total: Default::default(),
                _err: PhantomData::default(),
            }
        }
    }

    impl<Error> parallel::Reduce for Statistics<Error> {
        type Input = Result<(ChunkId, Vec<output::Entry>, Outcome), Error>;
        type FeedProduce = (ChunkId, Vec<output::Entry>);
        type Output = Outcome;
        type Error = Error;

        fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
            item.map(|(cid, entries, stats)| {
                self.total.aggregate(stats);
                (cid, entries)
            })
        }

        fn finalize(self) -> Result<Self::Output, Self::Error> {
            Ok(self.total)
        }
    }
}

mod types {
    use crate::data::output::entry;

    /// Information gathered during the run of [`iter_from_counts()`][super::iter_from_counts()].
    #[derive(Default, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Outcome {
        /// The amount of fully decoded objects. These are the most expensive as they are fully decoded.
        pub decoded_and_recompressed_objects: usize,
        /// The amount of objects that could be copied directly from the pack. These are cheapest as they
        /// only cost a memory copy for the most part.
        pub objects_copied_from_pack: usize,
    }

    impl Outcome {
        pub(in crate::data::output::entry) fn aggregate(
            &mut self,
            Outcome {
                decoded_and_recompressed_objects: decoded_objects,
                objects_copied_from_pack,
            }: Self,
        ) {
            self.decoded_and_recompressed_objects += decoded_objects;
            self.objects_copied_from_pack += objects_copied_from_pack;
        }
    }

    /// The way the iterator operates.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum Mode {
        /// Copy base objects and deltas from packs, while non-packed objects will be treated as base objects
        /// (i.e. without trying to delta compress them). This is a fast way of obtaining a back while benefitting
        /// from existing pack compression and spending the smallest possible time on compressing unpacked objects at
        /// the cost of bandwidth.
        PackCopyAndBaseObjects,
    }

    /// Configuration options for the pack generation functions provied in [this module][crate::data::output].
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Options {
        /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
        pub thread_limit: Option<usize>,
        /// The algorithm to produce a pack
        pub mode: Mode,
        /// If set, the resulting back can have deltas that refer to an object which is not in the pack. This can happen
        /// if the initial counted objects do not contain an object that an existing packed delta refers to, for example, because
        /// it wasn't part of the iteration, for instance when the iteration was performed on tree deltas or only a part of the
        /// commit graph. Please note that thin packs are not valid packs when stored on disk, thus they are only valid for packs
        /// that are being send over the wire.
        ///
        /// If set to false, delta objects will be decompressed and recompressed as base objects.
        pub allow_thin_pack: bool,
        /// The amount of objects per chunk or unit of work to be sent to threads for processing
        /// TODO: could this become the window size?
        pub chunk_size: usize,
        /// The pack data version to produce for each entry
        pub version: crate::data::Version,
    }

    impl Default for Options {
        fn default() -> Self {
            Options {
                thread_limit: None,
                mode: Mode::PackCopyAndBaseObjects,
                allow_thin_pack: false,
                chunk_size: 10,
                version: Default::default(),
            }
        }
    }

    /// The error returned by the pack generation function [`to_entry_iter()`][crate::data::output::entry::iter_from_counts()].
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
pub use types::{Error, Mode, Options, Outcome};
