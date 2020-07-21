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
                struct CacheEntry {
                    kind: Kind,
                    data: Vec<u8>,
                    compressed_size: usize,
                    hits_to_live: usize,
                }
                struct SharedCache<'a>(&'a mut BTreeMap<PackOffset, CacheEntry>);

                impl<'a> pack::cache::DecodeEntry for SharedCache<'a> {
                    fn put(&mut self, pack_offset: u64, data: &[u8], kind: Kind, compressed_size: usize) {
                        self.0.entry(pack_offset).or_insert_with(|| CacheEntry {
                            kind,
                            data: data.to_owned(),
                            compressed_size,
                            hits_to_live: 0,
                        });
                    }

                    fn get(&mut self, offset: u64, out: &mut Vec<u8>) -> Option<(Kind, usize)> {
                        let (res, should_delete) = if let Some(CacheEntry {
                            kind,
                            data,
                            compressed_size,
                            hits_to_live,
                        }) = self.0.get_mut(&offset)
                        {
                            out.resize(data.len(), 0);
                            out.copy_from_slice(&data);
                            *hits_to_live = hits_to_live.saturating_sub(1);
                            (Some((*kind, *compressed_size)), *hits_to_live == 0)
                        } else {
                            (None, false)
                        };
                        if should_delete {
                            self.0.remove(&offset);
                        }
                        res
                    }
                }
                impl<'a> SharedCache<'a> {
                    fn hits_to_live(&mut self, pack_offset: u64, hits: usize) {
                        if let Some(v) = self.0.get_mut(&pack_offset) {
                            v.hits_to_live += hits;
                        }
                    }
                }
                let mut cache = BTreeMap::new();

                let mut count = 0;
                while let Some(node) = nodes.pop() {
                    let pack_offset = node.pack_offset;
                    let index_entry = sorted_entries
                        .binary_search_by(|e| e.pack_offset.cmp(&pack_offset))
                        .expect("tree created by our sorted entries");
                    let index_entry_of_node = &sorted_entries[index_entry];

                    tree.children(node, &mut children);

                    let shared_cache = &mut SharedCache(&mut cache);
                    stats.push(self.process_entry(
                        mode,
                        pack,
                        shared_cache,
                        buf,
                        encode_buf,
                        progress,
                        &mut header_buf,
                        index_entry_of_node,
                    )?);
                    shared_cache.hits_to_live(pack_offset, children.len());

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
