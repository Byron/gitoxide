use super::{Error, Reducer};
use crate::pack::{self, data::decode, index, index::util};
use git_features::{
    parallel::{self, in_parallel_if},
    progress::{self, Progress},
};

/// Verify and validate the content of the index file
impl index::File {
    pub(crate) fn traverse_with_lookup<P, C>(
        &self,
        thread_limit: Option<usize>,
        mode: index::verify::Mode,
        make_cache: impl Fn() -> C + Send + Sync,
        mut root: progress::DoOrDiscard<P>,
        pack: &pack::data::File,
    ) -> Result<index::verify::Outcome, Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
    {
        let index_entries =
            util::index_entries_sorted_by_offset_ascending(self, root.add_child("collecting sorted index"));

        let (chunk_size, thread_limit, available_cores) =
            parallel::optimize_chunk_size_and_thread_limit(1000, Some(index_entries.len()), thread_limit, None);
        let there_are_enough_entries_to_process = || index_entries.len() > chunk_size * available_cores;
        let input_chunks = index_entries.chunks(chunk_size.max(chunk_size));
        let reduce_progress = std::sync::Mutex::new({
            let mut p = root.add_child("Checking");
            p.init(Some(self.num_objects()), Some("objects"));
            p
        });
        let state_per_thread = |index| {
            (
                make_cache(),
                Vec::with_capacity(2048), // decode buffer
                Vec::with_capacity(2048), // re-encode buffer
                reduce_progress.lock().unwrap().add_child(format!("thread {}", index)), // per thread progress
            )
        };

        in_parallel_if(
            there_are_enough_entries_to_process,
            input_chunks,
            thread_limit,
            state_per_thread,
            |entries: &[index::Entry],
             (cache, buf, encode_buf, progress)|
             -> Result<Vec<decode::Outcome>, index::verify::Error> {
                progress.init(Some(entries.len() as u32), Some("entries"));
                let mut stats = Vec::with_capacity(entries.len());
                let mut header_buf = [0u8; 64];
                for index_entry in entries.iter() {
                    stats.push(self.process_entry(
                        mode,
                        pack,
                        cache,
                        buf,
                        encode_buf,
                        progress,
                        &mut header_buf,
                        index_entry,
                    )?);
                    progress.inc();
                }
                Ok(stats)
            },
            Reducer::from_progress(&reduce_progress, pack.data_len()),
        )
    }
}
