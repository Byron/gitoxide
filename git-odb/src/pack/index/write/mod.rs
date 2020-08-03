use crate::{pack, pack::index::util::Chunks, pack::tree::Tree};
use git_features::{hash, parallel, parallel::in_parallel_if, progress::Progress};
use smallvec::alloc::collections::BTreeMap;
use std::{convert::TryInto, io};

mod encode;
mod error;
pub use error::Error;

mod types;
use types::{CacheEntry, Entry, ObjectKind, Reducer, TreeEntry};
pub use types::{EntrySlice, Mode, Outcome};

mod consume;
use consume::apply_deltas;

/// Various ways of writing an index file from pack entries
impl pack::index::File {
    /// Note that neither in-pack nor out-of-pack Ref Deltas are supported here, these must have been resolved beforehand.
    pub fn write_data_iter_to_stream<F, P>(
        kind: pack::index::Kind,
        mode: Mode<F>,
        entries: impl Iterator<Item = Result<pack::data::iter::Entry, pack::data::iter::Error>>,
        thread_limit: Option<usize>,
        mut root_progress: P,
        out: impl io::Write,
    ) -> Result<Outcome, Error>
    where
        F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
        P: Progress,
        <P as Progress>::SubProgress: Send,
    {
        if kind != pack::index::Kind::default() {
            return Err(Error::Unsupported(kind));
        }
        let mut num_objects: usize = 0;
        let mut bytes_to_process = 0u64;
        // This array starts out sorted by pack-offset
        let mut index_entries = Vec::with_capacity(entries.size_hint().0);
        let mut last_seen_trailer = None;
        let mut last_base_index = None;
        let mut first_delta_index = None;
        let mut cache_by_offset = BTreeMap::<_, CacheEntry>::new();
        let mut tree = Tree::new(entries.size_hint().0)?;
        let mut header_buf = [0u8; 16];
        let indexing_start = std::time::Instant::now();

        root_progress.init(Some(3), Some("steps"));
        root_progress.inc();
        let mut progress = root_progress.add_child("indexing");
        progress.init(entries.size_hint().1.map(|l| l as u32), Some("objects"));

        for (eid, entry) in entries.enumerate() {
            let pack::data::iter::Entry {
                header,
                pack_offset,
                header_size,
                compressed,
                decompressed,
                trailer,
            } = entry?;

            let compressed_len = compressed.len();
            bytes_to_process += decompressed.len() as u64;
            let entry_len = header_size as usize + compressed_len;
            let crc32 = {
                let header_len = header.to_write(decompressed.len() as u64, header_buf.as_mut())?;
                let state = hash::crc32_update(0, &header_buf[..header_len]);
                hash::crc32_update(state, &compressed)
            };

            use pack::data::Header::*;
            let (cache, kind) = match header {
                Blob | Tree | Commit | Tag => {
                    last_base_index = Some(eid);
                    tree.add_root(
                        pack_offset,
                        TreeEntry {
                            pack_offset,
                            entry_len,
                            kind: ObjectKind::Base(header.to_kind().expect("a base object")),
                            crc32,
                            cache: mode.base_cache(compressed.clone(), decompressed.clone()), // TODO: remove clones once owned
                        },
                    )?;
                    (
                        mode.base_cache(compressed, decompressed),
                        ObjectKind::Base(header.to_kind().expect("a base object")),
                    )
                }
                RefDelta { .. } => return Err(Error::IteratorInvariantNoRefDelta),
                OfsDelta { base_distance } => {
                    let base_pack_offset = pack::data::Header::verified_base_pack_offset(pack_offset, base_distance)
                        .ok_or_else(|| Error::IteratorInvariantBaseOffset(pack_offset, base_distance))?;
                    tree.add_child(
                        base_pack_offset,
                        pack_offset,
                        TreeEntry {
                            pack_offset,
                            entry_len,
                            kind: ObjectKind::OfsDelta(base_pack_offset),
                            crc32,
                            cache: mode.delta_cache(compressed.clone(), decompressed.clone()), // TODO: remove clones
                        },
                    )?;
                    cache_by_offset
                        .get_mut(&base_pack_offset)
                        .expect("this to work nice check is done in tree delete me")
                        .increment_child_count();

                    if first_delta_index.is_none() {
                        first_delta_index = Some(eid);
                    }

                    (
                        mode.delta_cache(compressed, decompressed),
                        ObjectKind::OfsDelta(base_pack_offset),
                    )
                }
            };

            cache_by_offset.insert(pack_offset, CacheEntry::new(cache));
            index_entries.push(Entry {
                pack_offset,
                entry_len: header_size as usize + compressed_len,
                kind,
                crc32,
            });
            last_seen_trailer = trailer;

            num_objects += 1;
            progress.inc();
        }
        progress.show_throughput(indexing_start, num_objects as u32, "objects");
        drop(progress);
        root_progress.inc();

        // Prevent us from trying to find bases for resolution past the point where they are
        let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(1, None, thread_limit, None);
        let last_base_index = last_base_index.ok_or(Error::IteratorInvariantBasesPresent)?;
        let num_objects: u32 = num_objects
            .try_into()
            .map_err(|_| Error::IteratorInvariantTooManyObjects(num_objects))?;

        // TODO: This COULD soon become a dashmap (in fast mode, or a Mutex protected shared map) as this will be edited
        // by threads to remove now unused caches.
        let cache_by_offset = parking_lot::Mutex::new(cache_by_offset);
        let reduce_progress = parking_lot::Mutex::new(root_progress.add_child("Resolving"));
        let sorted_pack_offsets_by_oid = {
            let mut items = in_parallel_if(
                || bytes_to_process > 5_000_000,
                Chunks {
                    iter: index_entries
                        .iter()
                        .take(last_base_index + 1)
                        .filter(|e| e.kind.is_base())
                        .cloned(),
                    size: chunk_size,
                }
                .zip(tree.into_chunks(chunk_size)),
                thread_limit,
                |thread_index| {
                    (
                        Vec::with_capacity(4096),
                        reduce_progress.lock().add_child(format!("thread {}", thread_index)),
                    )
                },
                |base_pack_offsets, state| {
                    apply_deltas(
                        base_pack_offsets,
                        state,
                        match first_delta_index {
                            Some(idx) => &index_entries[idx..],
                            None => &[],
                        },
                        &cache_by_offset,
                        &mode,
                        kind.hash(),
                    )
                },
                Reducer::new(num_objects, &reduce_progress),
            )?;
            items.sort_by_key(|e| e.1);
            items
        };

        drop(index_entries);
        drop(cache_by_offset);
        root_progress.inc();

        let pack_hash = last_seen_trailer.ok_or(Error::IteratorInvariantTrailer)?;
        let index_hash = encode::to_write(
            out,
            sorted_pack_offsets_by_oid,
            &pack_hash,
            kind,
            root_progress.add_child("writing index file"),
        )?;
        root_progress.inc();
        root_progress.show_throughput(indexing_start, num_objects, "objects");
        Ok(Outcome {
            index_kind: kind,
            index_hash,
            pack_hash,
            num_objects,
        })
    }
}
