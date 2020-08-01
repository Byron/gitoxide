use crate::{pack, pack::index::util::Chunks};
use git_features::{hash, parallel, parallel::in_parallel_if, progress::Progress};
use smallvec::alloc::collections::BTreeMap;
use std::{convert::TryInto, io};

mod encode;
mod error;
pub use error::Error;

mod types;
pub use types::*;

mod consume;
use consume::apply_deltas;

/// Various ways of writing an index file from pack entries
impl pack::index::File {
    /// Note that neither in-pack nor out-of-pack Ref Deltas are supported here, these must have been resolved beforehand.
    pub fn write_data_iter_to_stream<F>(
        kind: pack::index::Kind,
        mode: Mode<F>,
        entries: impl Iterator<Item = Result<pack::data::iter::Entry, pack::data::iter::Error>>,
        thread_limit: Option<usize>,
        _progress: impl Progress,
        out: impl io::Write,
    ) -> Result<Outcome, Error>
    where
        F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
    {
        if kind != pack::index::Kind::default() {
            return Err(Error::Unsupported(kind));
        }
        let mut num_objects = 0;
        let mut bytes_to_process = 0u64;
        // This array starts out sorted by pack-offset
        let mut index_entries = Vec::with_capacity(entries.size_hint().0);
        if index_entries.capacity() == 0 {
            return Err(Error::IteratorInvariantNonEmpty);
        }
        let mut last_seen_trailer = None;
        let mut last_base_index = None;
        let mut first_delta_index = None;
        let mut last_pack_offset = 0;
        let mut cache_by_offset = BTreeMap::<_, CacheEntry>::new();
        let mut header_buf = [0u8; 16];
        for (eid, entry) in entries.enumerate() {
            use pack::data::Header::*;

            let pack::data::iter::Entry {
                header,
                pack_offset,
                header_size,
                compressed,
                decompressed,
                trailer,
            } = entry?;
            let compressed_len = compressed.len();
            if pack_offset <= last_pack_offset {
                return Err(Error::IteratorInvariantIncreasingPackOffset(
                    last_pack_offset,
                    pack_offset,
                ));
            }
            last_pack_offset = pack_offset;
            num_objects += 1;
            bytes_to_process += decompressed.len() as u64;
            let crc32 = {
                let header_len = header.to_write(decompressed.len() as u64, header_buf.as_mut())?;
                let state = hash::crc32_update(0, &header_buf[..header_len]);
                hash::crc32_update(state, &compressed)
            };
            let (cache, kind) = match header {
                Blob | Tree | Commit | Tag => {
                    last_base_index = Some(eid);
                    (
                        mode.base_cache(compressed, decompressed),
                        ObjectKind::Base(header.to_kind().expect("a base object")),
                    )
                }
                RefDelta { .. } => return Err(Error::IteratorInvariantNoRefDelta),
                OfsDelta { base_distance } => {
                    let base_pack_offset = pack_offset - base_distance;
                    cache_by_offset
                        .get_mut(&base_pack_offset)
                        .ok_or_else(|| {
                            Error::IteratorInvariantBasesBeforeDeltasNeedThem(pack_offset, base_pack_offset)
                        })?
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
        }

        // Prevent us from trying to find bases for resolution past the point where they are
        let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(1, None, thread_limit, None);
        let last_base_index = last_base_index.ok_or(Error::IteratorInvariantBasesPresent)?;
        let num_objects: u32 = num_objects
            .try_into()
            .map_err(|_| Error::IteratorInvariantTooManyObjects(num_objects))?;

        // TODO: This COULD soon become a dashmap (in fast mode, or a Mutex protected shared map) as this will be edited
        // by threads to remove now unused caches. Probably also a good moment to switch to parking lot mutexes everywhere.
        let cache_by_offset = parking_lot::Mutex::new(cache_by_offset);
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
                },
                thread_limit,
                |_thread_index| Vec::with_capacity(4096),
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
                Reducer::new(num_objects),
            )?;
            items.sort_by_key(|e| e.1);
            items
        };

        drop(index_entries);
        drop(cache_by_offset);

        let pack_hash = last_seen_trailer.ok_or(Error::IteratorInvariantTrailer)?;
        let index_hash = encode::to_write(out, sorted_pack_offsets_by_oid, &pack_hash, kind)?;
        Ok(Outcome {
            index_kind: kind,
            index_hash,
            pack_hash,
            num_objects,
        })
    }
}
