use super::{Error, Reducer, SafetyCheck};
use crate::pack::{self, data::file, index, index::util};
use git_features::{
    interrupt::ResetOnDrop,
    parallel::{self, in_parallel_if},
    progress::{self, unit, Progress},
};

/// Verify and validate the content of the index file
impl index::File {
    /// Iterate through all _decoded objects_ in the given `pack` and handle them with a `Processor` using a cache to reduce the amount of
    /// waste while decoding objects.
    ///
    /// For more details, see the documentation on the [`traverse()`][index::File::traverse()] method.
    pub fn traverse_with_lookup<P, C, Processor, E>(
        &self,
        check: SafetyCheck,
        thread_limit: Option<usize>,
        new_processor: impl Fn() -> Processor + Send + Sync,
        new_cache: impl Fn() -> C + Send + Sync,
        mut progress: P,
        pack: &pack::data::File,
    ) -> Result<(git_hash::ObjectId, index::traverse::Outcome, P), Error<E>>
    where
        P: Progress,
        C: pack::cache::DecodeEntry,
        E: std::error::Error + Send + Sync + 'static,
        Processor: FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &mut <<P as Progress>::SubProgress as Progress>::SubProgress,
        ) -> Result<(), E>,
    {
        let _reset_interrupt = ResetOnDrop::default();
        let (verify_result, traversal_result) = parallel::join(
            {
                let pack_progress = progress.add_child("SHA1 of pack");
                let index_progress = progress.add_child("SHA1 of index");
                move || {
                    let res = self.possibly_verify(pack, check, pack_progress, index_progress);
                    if res.is_err() {
                        git_features::interrupt::trigger();
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
                let reduce_progress = parking_lot::Mutex::new({
                    let mut p = progress.add_child("Traversing");
                    p.init(Some(self.num_objects() as usize), progress::count("objects"));
                    p
                });
                let state_per_thread = |index| {
                    (
                        new_cache(),
                        new_processor(),
                        Vec::with_capacity(2048), // decode buffer
                        reduce_progress.lock().add_child(format!("thread {}", index)), // per thread progress
                    )
                };

                in_parallel_if(
                    there_are_enough_entries_to_process,
                    input_chunks,
                    thread_limit,
                    state_per_thread,
                    |entries: &[index::Entry],
                     (cache, ref mut processor, buf, progress)|
                     -> Result<Vec<file::decode_entry::Outcome>, Error<_>> {
                        progress.init(
                            Some(entries.len()),
                            Some(unit::dynamic(unit::Human::new(
                                unit::human::Formatter::new(),
                                "objects",
                            ))),
                        );
                        let mut stats = Vec::with_capacity(entries.len());
                        let mut header_buf = [0u8; 64];
                        for index_entry in entries.iter() {
                            let result = self.decode_and_process_entry(
                                check,
                                pack,
                                cache,
                                buf,
                                progress,
                                &mut header_buf,
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
                    Reducer::from_progress(&reduce_progress, pack.data_len(), check),
                )
            },
        );
        let id = verify_result?;
        let res = traversal_result?;
        Ok((id, res, progress))
    }
}
