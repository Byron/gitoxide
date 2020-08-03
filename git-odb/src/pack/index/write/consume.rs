use crate::{
    hash, loose, pack,
    pack::index::write::types::{Bytes, Cache, CacheEntry, Entry, EntrySlice, Mode, ObjectKind},
    pack::index::write::Error,
    zlib,
};
use git_features::progress::Progress;
use git_object::{owned, HashKind};
use smallvec::alloc::collections::BTreeMap;
use std::{cell::RefCell, io};

pub(crate) fn apply_deltas<F, P>(
    (mut base_entries, _): (Vec<Entry>, Vec<pack::tree::Node<pack::index::write::types::TreeEntry>>),
    (bytes_buf, progress): &mut (Vec<u8>, P),
    entries: &[Entry],
    caches: &parking_lot::Mutex<BTreeMap<u64, CacheEntry>>,
    mode: &Mode<F>,
    hash_kind: HashKind,
) -> Result<Vec<(u64, owned::Id, u32)>, Error>
where
    F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
    P: Progress,
{
    let local_caches = RefCell::new(BTreeMap::<u64, CacheEntry>::new());
    enum FetchMode {
        /// Bases for deltas will decrement their refcount.
        AsBase,
        /// Sources will be fetch as - is, without reducing their count.
        AsSource,
    }
    let bytes_buf = RefCell::new(bytes_buf);
    let decompressed_bytes_from_cache =
        |pack_offset: &u64, entry_size: &usize, fetch: FetchMode| -> Result<(bool, Vec<u8>), Error> {
            let cache = {
                // get the entry from the local cache, and on miss, pull it out of the expensive,
                // shared & locked cache.
                let mut local_caches = local_caches.borrow_mut();
                let c = local_caches.entry(*pack_offset).or_insert_with(|| {
                    caches
                        .lock()
                        .remove_entry(&pack_offset)
                        .expect("an entry for every pack offset")
                        .1
                });
                match fetch {
                    FetchMode::AsSource => c.cache(),
                    FetchMode::AsBase => c.cache_decr(),
                }
            };
            let (is_borrowed, cache) = match cache {
                Bytes::Borrowed(b) => (true, b),
                Bytes::Owned(b) => (false, b),
            };
            let bytes = match cache {
                Cache::Decompressed(b) => b,
                Cache::Compressed(b, decompressed_len) => decompress_all_at_once(&b, decompressed_len)?,
                Cache::Unset => {
                    let mut bytes_buf = bytes_buf.borrow_mut();
                    bytes_buf.resize(*entry_size, 0);
                    match mode {
                        Mode::ResolveDeltas(r) | Mode::ResolveBases(r) | Mode::ResolveBasesAndDeltas(r) => {
                            r(*pack_offset..*pack_offset + *entry_size as u64, &mut bytes_buf)
                                .ok_or_else(|| Error::ConsumeResolveFailed(*pack_offset))?;
                            let entry = pack::data::Entry::from_bytes(&bytes_buf, *pack_offset);
                            decompress_all_at_once(
                                &bytes_buf[entry.header_size() as usize..],
                                entry.decompressed_size as usize,
                            )?
                        }
                        Mode::InMemoryDecompressed | Mode::InMemory => {
                            unreachable!("BUG: If there is no cache, we always need a resolver")
                        }
                    }
                }
            };
            Ok((is_borrowed, bytes))
        };
    let possibly_return_to_cache = |pack_offset: &u64, is_borrowed: bool, bytes: Vec<u8>| {
        if is_borrowed {
            local_caches
                .borrow_mut()
                .get_mut(pack_offset)
                .expect("an entry for every pack offset")
                .set_decompressed(bytes);
        }
    };
    let compute_hash = |kind: git_object::Kind, bytes: &[u8]| -> owned::Id {
        let mut write = hash::Write::new(io::sink(), hash_kind);
        loose::object::header::encode(kind, bytes.len() as u64, &mut write)
            .expect("write to sink and hash cannot fail");
        write.hash.update(bytes);
        owned::Id::from(write.hash.digest())
    };
    let mut out = Vec::with_capacity(base_entries.len()); // perfectly conservative guess

    // Compute hashes for all of our bases right away
    progress.init(None, Some("objects"));
    for Entry {
        pack_offset,
        kind,
        entry_len,
        crc32,
    } in &base_entries
    {
        let (is_borrowed, base_bytes) = decompressed_bytes_from_cache(pack_offset, entry_len, FetchMode::AsSource)?;
        out.push((
            *pack_offset,
            compute_hash(kind.to_kind().expect("base object"), &base_bytes),
            *crc32,
        ));
        possibly_return_to_cache(pack_offset, is_borrowed, base_bytes);
        progress.inc();
    }

    // find all deltas that match our bases, decompress them, apply them to the decompressed base, keep the hash
    // and finally store the fully decompressed delta as new base (if they have dependants of their own).
    // If there is nobody else using them, we could remove them, but that's expensive so let's just keep them around.
    progress.init(None, Some("objects"));
    for Entry {
        pack_offset,
        entry_len,
        kind,
        crc32,
    } in entries
    {
        let base_pack_offset = match kind {
            ObjectKind::Base(_) => continue, // we only work on deltas, bases can be intermixed though
            ObjectKind::OfsDelta(ofs) => ofs,
        };
        let base_index = match base_entries.binary_search_by_key(&base_pack_offset, |base| &base.pack_offset) {
            Ok(idx) => idx,
            Err(_) => continue, // not our delta
        };

        let base_entry = &base_entries[base_index];
        let (base_is_borrowed, base_bytes) =
            decompressed_bytes_from_cache(&base_entry.pack_offset, &base_entry.entry_len, FetchMode::AsBase)?;
        let (delta_is_borrowed, delta_bytes) =
            decompressed_bytes_from_cache(pack_offset, entry_len, FetchMode::AsSource)?;
        let (base_size, consumed) = pack::data::decode::delta_header_size_ofs(&delta_bytes);
        let mut header_ofs = consumed;
        assert_eq!(
            base_bytes.len(),
            base_size as usize,
            "recorded base size in delta does not match"
        );
        let (result_size, consumed) = pack::data::decode::delta_header_size_ofs(&delta_bytes[consumed..]);
        header_ofs += consumed;

        let mut fully_resolved_delta_bytes = bytes_buf.borrow_mut();
        fully_resolved_delta_bytes.resize(result_size as usize, 0);
        pack::data::decode::apply_delta(&base_bytes, &mut fully_resolved_delta_bytes, &delta_bytes[header_ofs..]);

        possibly_return_to_cache(&base_entry.pack_offset, base_is_borrowed, base_bytes);

        out.push((
            *pack_offset,
            compute_hash(
                base_entry.kind.to_kind().expect("base always has object kind"),
                &fully_resolved_delta_bytes,
            ),
            *crc32,
        ));
        let delta_data_to_return = if delta_is_borrowed {
            let delta_entry = Entry {
                pack_offset: *pack_offset,
                kind: base_entry.kind.clone(),
                entry_len: 0,
                crc32: 0,
            };
            base_entries.insert(
                base_entries
                    .binary_search_by_key(pack_offset, |e| e.pack_offset)
                    .expect_err("Delta has not yet been added"),
                delta_entry,
            );
            // we will be a base ourselves, so assure our cache contains the fully decompressed version of ourselves
            fully_resolved_delta_bytes.to_owned()
        } else {
            delta_bytes
        };
        possibly_return_to_cache(pack_offset, delta_is_borrowed, delta_data_to_return);
        progress.inc();
    }
    out.shrink_to_fit();
    Ok(out)
}

fn decompress_all_at_once(b: &[u8], decompressed_len: usize) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    out.resize(decompressed_len, 0);
    zlib::Inflate::default()
        .once(&b, &mut io::Cursor::new(&mut out), true)
        .map_err(|err| Error::ConsumeZlibInflate(err, "Failed to decompress entry"))?;
    Ok(out)
}
