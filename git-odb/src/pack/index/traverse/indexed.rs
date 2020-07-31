use super::{Error, Reducer, SafetyCheck};
use crate::{
    pack,
    pack::index::access::PackOffset,
    pack::index::{
        self,
        util::{index_entries_sorted_by_offset_ascending, Chunks},
    },
};
use git_features::{
    parallel::{self, in_parallel_if},
    progress::Progress,
};
use git_object::Kind;
use std::collections::BTreeMap;

impl index::File {
    pub(crate) fn traverse_with_index_lookup<P, Processor>(
        &self,
        check: SafetyCheck,
        thread_limit: Option<usize>,
        new_processor: impl Fn() -> Processor + Send + Sync,
        mut root: P,
        pack: &pack::data::File,
    ) -> Result<(index::traverse::Outcome, P), Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        Processor: FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &pack::data::decode::Outcome,
            &mut <<P as Progress>::SubProgress as Progress>::SubProgress,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
    {
        let sorted_entries = index_entries_sorted_by_offset_ascending(self, root.add_child("collecting sorted index"));
        let tree = pack::graph::DeltaTree::from_sorted_offsets(
            sorted_entries.iter().map(|e| e.pack_offset),
            pack.path(),
            root.add_child("indexing"),
            |id| self.lookup(id).map(|idx| self.pack_offset_at_index(idx)),
        )?;
        let if_there_are_enough_objects = || self.num_objects > 10_000;

        let reduce_progress = std::sync::Mutex::new({
            let mut p = root.add_child("Iterating");
            p.init(Some(self.num_objects()), Some("objects"));
            p
        });

        let state_per_thread = |index| {
            (
                new_processor(),
                Vec::<u8>::with_capacity(2048), // decode buffer
                Vec::<(pack::graph::Node, u32)>::new(),
                reduce_progress.lock().unwrap().add_child(format!("thread {}", index)), // per thread progress
            )
        };
        let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(1, None, thread_limit, None);
        in_parallel_if(
            if_there_are_enough_objects,
            Chunks {
                size: chunk_size,
                iter: tree.bases(),
            },
            thread_limit,
            state_per_thread,
            |input: Vec<pack::graph::Node>,
             (ref mut processor, buf, nodes, progress)|
             -> Result<Vec<pack::data::decode::Outcome>, Error> {
                let mut stats = Vec::new();
                let mut header_buf = [0u8; 64];
                let mut children = Vec::new();
                progress.init(None, Some("entries"));

                struct CacheEntry {
                    kind: Kind,
                    data: Vec<u8>,
                    compressed_size: usize,
                }
                struct SharedCache<'a>(&'a mut BTreeMap<PackOffset, CacheEntry>);

                impl<'a> pack::cache::DecodeEntry for SharedCache<'a> {
                    fn put(&mut self, pack_offset: u64, data: &[u8], kind: Kind, compressed_size: usize) {
                        self.0.entry(pack_offset).or_insert_with(|| CacheEntry {
                            kind,
                            data: data.to_owned(),
                            compressed_size,
                        });
                    }

                    fn get(&mut self, offset: u64, out: &mut Vec<u8>) -> Option<(Kind, usize)> {
                        self.0.get_mut(&offset).map(
                            |CacheEntry {
                                 kind,
                                 data,
                                 compressed_size,
                             }| {
                                out.resize(data.len(), 0);
                                out.copy_from_slice(&data);
                                (*kind, *compressed_size)
                            },
                        )
                    }
                }

                for node in input {
                    nodes.clear();
                    nodes.push((node, 0));
                    let mut cache = BTreeMap::new();

                    while let Some((node, level)) = nodes.pop() {
                        let pack_offset = node.pack_offset;
                        let index_entry = sorted_entries
                            .binary_search_by(|e| e.pack_offset.cmp(&pack_offset))
                            .expect("tree created by our sorted entries");
                        let index_entry_of_node = &sorted_entries[index_entry];

                        tree.children(node, &mut children);

                        let shared_cache = &mut SharedCache(&mut cache);
                        let result = self.process_entry_dispatch(
                            check,
                            pack,
                            shared_cache,
                            buf,
                            progress,
                            &mut header_buf,
                            index_entry_of_node,
                            processor,
                        );
                        progress.inc();
                        nodes.extend(children.iter().cloned().map(|cn| (cn, level + 1)));

                        let mut stat = match result {
                            Err(err @ Error::PackDecode(_, _, _)) if !check.fatal_decode_error() => {
                                progress.info(format!("Ignoring decode error: {}", err));
                                continue;
                            }
                            res => res,
                        }?;
                        stat.num_deltas = level;
                        stats.push(stat);
                    }
                }

                Ok(stats)
            },
            Reducer::from_progress(&reduce_progress, pack.data_len(), check),
        )
        .map(|res| (res, root))
    }
}
