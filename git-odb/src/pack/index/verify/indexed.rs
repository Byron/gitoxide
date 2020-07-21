use super::{Error, Mode, Outcome};
use crate::{
    pack,
    pack::index::access::PackOffset,
    pack::index::{self, verify::util},
};
use git_features::{
    parallel::in_parallel_if,
    progress::{self, Progress},
};
use git_object::Kind;
use std::collections::BTreeMap;

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
                struct SharedCache<'a>(&'a mut BTreeMap<PackOffset, (Kind, Vec<u8>, usize)>);

                impl<'a> pack::cache::DecodeEntry for SharedCache<'a> {
                    fn put(&mut self, pack_offset: u64, data: &[u8], kind: Kind, compressed_size: usize) {
                        self.0
                            .entry(pack_offset)
                            .or_insert_with(|| (kind, data.to_owned(), compressed_size));
                    }

                    fn get(&mut self, offset: u64, out: &mut Vec<u8>) -> Option<(Kind, usize)> {
                        self.0.get(&offset).map(|(kind, data, compressed_size)| {
                            out.resize(data.len(), 0);
                            out.copy_from_slice(&data);
                            (*kind, *compressed_size)
                        })
                    }
                }
                let mut cache = BTreeMap::new();

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
                        &mut SharedCache(&mut cache),
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
