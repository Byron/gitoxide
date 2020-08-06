use super::{Error, Reducer, SafetyCheck};
use crate::pack::{self, data::decode, index, index::util};
use git_features::{
    parallel::{self, in_parallel_if},
    progress::Progress,
};

/// Verify and validate the content of the index file
impl index::File {
    pub(crate) fn traverse_with_lookup<P, C, Processor>(
        &self,
        check: SafetyCheck,
        thread_limit: Option<usize>,
        new_processor: impl Fn() -> Processor + Send + Sync,
        make_cache: impl Fn() -> C + Send + Sync,
        mut root: P,
        pack: &pack::data::File,
    ) -> Result<(index::traverse::Outcome, P), Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
        Processor: FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &mut <<P as Progress>::SubProgress as Progress>::SubProgress,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
    {
        let (chunk_size, thread_limit, _) =
            parallel::optimize_chunk_size_and_thread_limit(1000, Some(self.num_objects as usize), thread_limit, None);
        let there_are_enough_entries_to_process = || self.num_objects > 10_000;
        let reduce_progress = parking_lot::Mutex::new({
            let mut p = root.add_child("Traversing");
            p.init(Some(self.num_objects()), Some("objects"));
            p
        });
        let state_per_thread = |index| {
            (
                make_cache(),
                new_processor(),
                Vec::with_capacity(2048),                                      // decode buffer
                reduce_progress.lock().add_child(format!("thread {}", index)), // per thread progress
            )
        };

        in_parallel_if(
            there_are_enough_entries_to_process,
            util::Chunks {
                iter: self.iter(),
                size: chunk_size,
            },
            thread_limit,
            state_per_thread,
            |entries: Vec<index::Entry>,
             (cache, ref mut processor, buf, progress)|
             -> Result<Vec<decode::Outcome>, Error> {
                progress.init(Some(entries.len() as u32), Some("entries"));
                let mut stats = Vec::with_capacity(entries.len());
                let mut header_buf = [0u8; 64];
                for index_entry in entries.iter() {
                    let result = self.process_entry_dispatch(
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
                        Err(err @ Error::PackDecode(_, _, _)) if !check.fatal_decode_error() => {
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
        .map(|res| (res, root))
    }
}
