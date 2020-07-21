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
        mode: Mode,
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
                Vec::<u8>::with_capacity(2048), // decode buffer
                Vec::<u8>::with_capacity(2048), // re-encode buffer
                Vec::<pack::graph::Node>::new(),
                reduce_progress.lock().unwrap().add_child(format!("thread {}", index)), // per thread progress
            )
        };
        in_parallel_if(
            if_there_are_enough_objects,
            tree.bases(),
            thread_limit,
            state_per_thread,
            |node: pack::graph::Node,
             (buf, encode_buf, nodes, progress): &mut (Vec<u8>, Vec<u8>, Vec<pack::graph::Node>, _)|
             -> Result<Vec<pack::data::decode::Outcome>, Error> {
                let mut stats = Vec::new();
                let mut header_buf = [0u8; 64];
                nodes.clear();
                nodes.push(node);
                let mut children = Vec::new();

                let mut count = 0;
                while let Some(node) = nodes.pop() {
                    let index_entry = sorted_entries
                        .binary_search_by(|e| e.pack_offset.cmp(&node.pack_offset))
                        .expect("tree created by our sorted entries");
                    let index_entry = &sorted_entries[index_entry];

                    tree.children(node, &mut children);

                    stats.push(self.process_entry(
                        mode,
                        pack,
                        &mut pack::cache::DecodeEntryNoop,
                        buf,
                        encode_buf,
                        progress,
                        &mut header_buf,
                        index_entry,
                    )?);

                    count += 1;
                    progress.set(count);
                    nodes.extend(children.iter().cloned());
                }

                Ok(stats)
            },
            index::verify::Reducer::from_progress(&reduce_progress, pack.data_len()),
        )
    }
}
