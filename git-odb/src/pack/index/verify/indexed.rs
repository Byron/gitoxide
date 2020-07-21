use super::{Error, Mode, Outcome};
use crate::{
    pack,
    pack::index::{self, verify::util},
};
use git_features::{
    parallel::in_parallel_if,
    progress::{self, Progress},
};

impl index::File {
    pub(crate) fn inner_verify_with_indexed_lookup<P, C>(
        &self,
        thread_limit: Option<usize>,
        _mode: Mode,
        _make_cache: impl Fn() -> C + Send + Sync,
        mut root: progress::DoOrDiscard<P>,
        pack: &pack::data::File,
    ) -> Result<Outcome, Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
    {
        let sorted_entries =
            util::index_entries_sorted_by_offset_ascending(self, root.add_child("collecting sorted index"));
        let tree = pack::graph::DeltaTree::from_sorted_offsets(
            sorted_entries.iter().map(|e| e.pack_offset),
            pack.path(),
            root.add_child("indexing"),
        )?;
        let if_there_are_enough_objects = || self.num_objects > 10_000;

        let reduce_progress = std::sync::Mutex::new({
            let mut p = root.add_child("Checking");
            p.init(Some(self.num_objects()), Some("objects"));
            p
        });

        let state_per_thread = |index| {
            (
                Vec::<u8>::with_capacity(2048),                                         // decode buffer
                Vec::<u8>::with_capacity(2048),                                         // re-encode buffer
                reduce_progress.lock().unwrap().add_child(format!("thread {}", index)), // per thread progress
            )
        };
        in_parallel_if(
            if_there_are_enough_objects,
            tree.bases(),
            thread_limit,
            state_per_thread,
            |_node: pack::graph::Node, (_buf, _encode_buf, _progress)| Ok::<_, Error>(Vec::new()),
            index::verify::Reducer::from_progress(&reduce_progress, pack.data_len()),
        )
    }
}
