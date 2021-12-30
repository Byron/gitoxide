use git_features::{
    parallel::{self, in_parallel_if},
    progress::{self, unit, Progress},
};

use super::{Error, Reducer};
use crate::{data, index, index::util};

mod options {
    use std::sync::atomic::AtomicBool;

    use crate::index::traverse::SafetyCheck;

    /// Traversal options for [`traverse()`][crate::index::File::traverse_with_lookup()]
    #[derive(Debug, Clone)]
    pub struct Options<'a> {
        /// If `Some`, only use the given amount of threads. Otherwise, the amount of threads to use will be selected based on
        /// the amount of available logical cores.
        pub thread_limit: Option<usize>,
        /// The kinds of safety checks to perform.
        pub check: SafetyCheck,
        /// A flag to indicate whether the algorithm should be interrupted. Will be checked occasionally allow stopping a running
        /// computation.
        pub should_interrupt: &'a AtomicBool,
    }
}
use std::sync::atomic::Ordering;

use crate::index::traverse::Outcome;
use git_features::threading::{lock, Mutable, OwnShared};
pub use options::Options;

/// Verify and validate the content of the index file
impl index::File {
    /// Iterate through all _decoded objects_ in the given `pack` and handle them with a `Processor` using a cache to reduce the amount of
    /// waste while decoding objects.
    ///
    /// For more details, see the documentation on the [`traverse()`][index::File::traverse()] method.
    pub fn traverse_with_lookup<P, C, Processor, E>(
        &self,
        new_processor: impl Fn() -> Processor + Send + Clone,
        new_cache: impl Fn() -> C + Send + Clone,
        mut progress: P,
        pack: &crate::data::File,
        Options {
            thread_limit,
            check,
            should_interrupt,
        }: Options<'_>,
    ) -> Result<Outcome<P>, Error<E>>
    where
        P: Progress,
        C: crate::cache::DecodeEntry,
        E: std::error::Error + Send + Sync + 'static,
        Processor: FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &mut <<P as Progress>::SubProgress as Progress>::SubProgress,
        ) -> Result<(), E>,
    {
        let (verify_result, traversal_result) = parallel::join(
            {
                let pack_progress = progress.add_child(format!(
                    "Hash of pack '{}'",
                    pack.path().file_name().expect("pack has filename").to_string_lossy()
                ));
                let index_progress = progress.add_child(format!(
                    "Hash of index '{}'",
                    self.path.file_name().expect("index has filename").to_string_lossy()
                ));
                move || {
                    let res = self.possibly_verify(pack, check, pack_progress, index_progress, should_interrupt);
                    if res.is_err() {
                        should_interrupt.store(true, Ordering::SeqCst);
                    }
                    res
                }
            },
            || {
                let index_entries =
                    util::index_entries_sorted_by_offset_ascending(self, progress.add_child("collecting sorted index"));

                let (chunk_size, thread_limit, available_cores) =
                    parallel::optimize_chunk_size_and_thread_limit(1000, Some(index_entries.len()), thread_limit, None);
                let there_are_enough_entries_to_process = || index_entries.len() > chunk_size * available_cores;
                let input_chunks = index_entries.chunks(chunk_size.max(chunk_size));
                let reduce_progress = OwnShared::new(Mutable::new({
                    let mut p = progress.add_child("Traversing");
                    p.init(Some(self.num_objects() as usize), progress::count("objects"));
                    p
                }));
                let state_per_thread = {
                    let reduce_progress = reduce_progress.clone();
                    move |index| {
                        (
                            new_cache(),
                            new_processor(),
                            Vec::with_capacity(2048), // decode buffer
                            lock(&reduce_progress).add_child(format!("thread {}", index)), // per thread progress
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
                     -> Result<Vec<data::decode_entry::Outcome>, Error<_>> {
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
