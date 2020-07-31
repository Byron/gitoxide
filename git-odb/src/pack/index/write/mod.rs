use crate::{hash, pack, pack::index::util::Chunks, pack::index::V2_SIGNATURE};
use byteorder::{BigEndian, WriteBytesExt};
use git_features::{parallel, parallel::in_parallel_if, progress::Progress};
use git_object::owned;
use smallvec::alloc::collections::BTreeMap;
use std::io;

mod error;
pub use error::Error;

mod types;
use std::convert::TryInto;
pub use types::*;

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
        F: for<'r> Fn(u64, &'r mut Vec<u8>) -> Option<(pack::data::Header, u64)>,
    {
        use io::Write;

        let mut out = hash::Write::new(out, kind.hash());
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
        let mut last_pack_offset = 0;
        // TODO: This should soon become a dashmap (in fast mode, or a Mutex protected shared map) as this will be edited
        // by threads to remove now unused caches. Probably also a good moment to switch to parking lot mutexes everywhere.
        let mut cache_by_offset = BTreeMap::<_, CacheEntry>::new();
        for (eid, entry) in entries.enumerate() {
            let pack::data::iter::Entry {
                header,
                pack_offset,
                header_size: _,
                compressed,
                decompressed,
                trailer,
            } = entry?;
            if !(pack_offset > last_pack_offset) {
                return Err(Error::IteratorInvariantIncreasingPackOffset(
                    last_pack_offset,
                    pack_offset,
                ));
            }
            last_pack_offset = pack_offset;
            use pack::data::Header::*;
            num_objects += 1;
            bytes_to_process += decompressed.len() as u64;
            let (cache, _is_base) = match header {
                Blob | Tree | Commit | Tag => {
                    last_base_index = Some(eid);
                    (mode.base_cache(compressed, decompressed), true)
                }
                RefDelta { .. } => return Err(Error::RefDelta),
                OfsDelta {
                    pack_offset: base_pack_offset,
                } => {
                    cache_by_offset
                        .get_mut(&base_pack_offset)
                        .ok_or_else(|| {
                            Error::IteratorInvariantBasesBeforeDeltasNeedThem(pack_offset, base_pack_offset)
                        })?
                        .child_count += 1;
                    (mode.delta_cache(compressed, decompressed), false)
                }
            };

            cache_by_offset.insert(
                pack_offset,
                CacheEntry {
                    child_count: 0,
                    _cache: cache,
                },
            );
            index_entries.push(Entry {
                pack_offset: pack_offset,
                is_base: _is_base,
                crc32: 0, // TBD, but can be done right here, needs header encoding
            });
            last_seen_trailer = trailer;
        }

        // Prevent us from trying to find bases for resolution past the point where they are
        let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(1, None, thread_limit, None);
        let last_base_index = last_base_index.ok_or(Error::IteratorInvariantBasesPresent)?;
        let num_objects: u32 = num_objects
            .try_into()
            .map_err(|_| Error::IteratorInvariantTooManyObjects(num_objects))?;
        let mut sorted_pack_offsets_by_oid = {
            let mut items = in_parallel_if(
                || bytes_to_process > 5_000_000,
                Chunks {
                    iter: index_entries.iter().take(last_base_index).filter(|e| e.is_base),
                    size: chunk_size,
                },
                thread_limit,
                |_| (),
                |_base_pack_offsets, _state| Vec::new(),
                Reducer::new(num_objects),
            )?;
            items.sort_by_key(|e| e.1);
            items
        };

        // Bring crc32 back into our perfectly sorted oid which is sorted by oid
        for (pack_offset, _oid, crc32) in sorted_pack_offsets_by_oid.iter_mut() {
            let index = index_entries
                .binary_search_by_key(pack_offset, |e| e.pack_offset)
                .expect("both arrays to have the same pack-offsets");
            *crc32 = index_entries[index].crc32;
        }
        drop(index_entries);

        // Write header
        out.write_all(V2_SIGNATURE)?;
        out.write_u32::<BigEndian>(kind as u32)?;

        // todo: write fanout

        let _index_hash = out.hash.digest();
        Ok(Outcome {
            index_kind: kind,
            index_hash: owned::Id::from([0u8; 20]),
            pack_hash: last_seen_trailer.ok_or(Error::IteratorInvariantTrailer)?,
            num_objects,
        })
    }
}
