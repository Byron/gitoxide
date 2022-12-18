use std::sync::atomic::{AtomicBool, Ordering};

use git_features::{
    parallel::{self, in_parallel_if},
    progress::{self, unit, Progress},
};

use super::{Error, Reducer};
use crate::{data, index, index::util};

/// Traversal options for [`traverse()`][crate::index::File::traverse_with_lookup()]
pub struct Options<F> {
    /// If `Some`, only use the given amount of threads. Otherwise, the amount of threads to use will be selected based on
    /// the amount of available logical cores.
    pub thread_limit: Option<usize>,
    /// The kinds of safety checks to perform.
    pub check: crate::index::traverse::SafetyCheck,
    /// A function to create a pack cache
    pub make_pack_lookup_cache: F,
}

impl Default for Options<fn() -> crate::cache::Never> {
    fn default() -> Self {
        Options {
            check: Default::default(),
            thread_limit: None,
            make_pack_lookup_cache: || crate::cache::Never,
        }
    }
}

use git_features::threading::{lock, Mutable, OwnShared};

use crate::index::traverse::Outcome;

/// Verify and validate the content of the index file
impl index::File {
    /// Iterate through all _decoded objects_ in the given `pack` and handle them with a `Processor` using a cache to reduce the amount of
    /// waste while decoding objects.
    ///
    /// For more details, see the documentation on the [`traverse()`][index::File::traverse()] method.
    pub fn traverse_with_lookup<P, C, Processor, E, F>(
        &self,
        new_processor: impl Fn() -> Processor + Send + Clone,
        pack: &crate::data::File,
        mut progress: P,
        should_interrupt: &AtomicBool,
        Options {
            thread_limit,
            check,
            make_pack_lookup_cache,
        }: Options<F>,
    ) -> Result<Outcome<P>, Error<E>>
    where
        P: Progress,
        C: crate::cache::DecodeEntry,
        E: std::error::Error + Send + Sync + 'static,
        Processor: FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &mut <P::SubProgress as Progress>::SubProgress,
        ) -> Result<(), E>,
        F: Fn() -> C + Send + Clone,
    {
        let (verify_result, traversal_result) = parallel::join(
            {
                let pack_progress = progress.add_child_with_id(
                    format!(
                        "Hash of pack '{}'",
                        pack.path().file_name().expect("pack has filename").to_string_lossy()
                    ),
                    *b"PTHP", /* Pack Traverse Hash Pack bytes */
                );
                let index_progress = progress.add_child_with_id(
                    format!(
                        "Hash of index '{}'",
                        self.path.file_name().expect("index has filename").to_string_lossy()
                    ),
                    *b"PTHI", /* Pack Traverse Hash Index bytes */
                );
                move || {
                    let res = self.possibly_verify(pack, check, pack_progress, index_progress, should_interrupt);
                    if res.is_err() {
                        should_interrupt.store(true, Ordering::SeqCst);
                    }
                    res
                }
            },
            || {
                let index_entries = util::index_entries_sorted_by_offset_ascending(
                    self,
                    progress.add_child_with_id("collecting sorted index", *b"PTCE"),
                ); /* Pack Traverse Collect sorted Entries */

                let (chunk_size, thread_limit, available_cores) =
                    parallel::optimize_chunk_size_and_thread_limit(1000, Some(index_entries.len()), thread_limit, None);
                let there_are_enough_entries_to_process = || index_entries.len() > chunk_size * available_cores;
                let input_chunks = index_entries.chunks(chunk_size.max(chunk_size));
                let reduce_progress = OwnShared::new(Mutable::new({
                    let mut p = progress.add_child_with_id("Traversing", *b"PTRO"); /* Pack Traverse Resolve Objects */
                    p.init(Some(self.num_objects() as usize), progress::count("objects"));
                    p
                }));
                let state_per_thread = {
                    let reduce_progress = reduce_progress.clone();
                    move |index| {
                        (
                            make_pack_lookup_cache(),
                            new_processor(),
                            Vec::with_capacity(2048), // decode buffer
                            lock(&reduce_progress)
                                .add_child_with_id(format!("thread {}", index), git_features::progress::UNKNOWN), // per thread progress
                        )
                    }
                };

                in_parallel_if(
                    there_are_enough_entries_to_process,
                    input_chunks,
                    thread_limit,
                    state_per_thread,
                    |entries: &[index::Entry],
                     (cache, ref mut processor, buf, progress)|
                     -> Result<Vec<data::decode::entry::Outcome>, Error<_>> {
                        progress.init(
                            Some(entries.len()),
                            Some(unit::dynamic(unit::Human::new(
                                unit::human::Formatter::new(),
                                "objects",
                            ))),
                        );
                        let mut stats = Vec::with_capacity(entries.len());
                        progress.set(0);
                        for index_entry in entries.iter() {
                            let result = self.decode_and_process_entry(
                                check,
                                pack,
                                cache,
                                buf,
                                progress,
                                index_entry,
                                processor,
                            );
                            progress.inc();
                            let stat = match result {
                                Err(err @ Error::PackDecode { .. }) if !check.fatal_decode_error() => {
                                    progress.info(format!("Ignoring decode error: {}", err));
                                    continue;
                                }
                                res => res,
                            }?;
                            stats.push(stat);
                        }
                        Ok(stats)
                    },
                    Reducer::from_progress(reduce_progress, pack.data_len(), check, should_interrupt),
                )
            },
        );
        Ok(Outcome {
            actual_index_checksum: verify_result?,
            statistics: traversal_result?,
            progress,
        })
    }
}
