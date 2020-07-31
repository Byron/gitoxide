use crate::{
    hash, loose, pack,
    pack::index::write::{Bytes, Cache, CacheEntry, Entry, EntrySlice, Error, Mode},
    zlib,
};
use git_object::{owned, HashKind};
use smallvec::alloc::collections::BTreeMap;
use std::io;

pub(crate) fn apply_deltas<F>(
    base_entries: Vec<Entry>,
    resolve_buf: &mut Vec<u8>,
    _entries: &[Entry],
    caches: &parking_lot::Mutex<BTreeMap<u64, CacheEntry>>,
    mode: &Mode<F>,
    hash_kind: HashKind,
) -> Result<Vec<(u64, owned::Id)>, Error>
where
    F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
{
    let mut decompressed_bytes_from_cache = |pack_offset: &u64, entry_size: &usize| -> Result<(bool, Vec<u8>), Error> {
        let cache = caches
            .lock()
            .get_mut(&pack_offset)
            .expect("an entry for every pack offset")
            .cache();
        let (is_borrowed, cache) = match cache {
            Bytes::Borrowed(b) => (true, b),
            Bytes::Owned(b) => (false, b),
        };
        let bytes = match cache {
            Cache::Decompressed(b) => b,
            Cache::Compressed(b, decompressed_len) => decompress_all_at_once(&b, decompressed_len)?,
            Cache::Unset => {
                resolve_buf.resize(*entry_size, 0);
                match mode {
                    Mode::ResolveDeltas(r) | Mode::ResolveBases(r) | Mode::ResolveBasesAndDeltas(r) => {
                        r(*pack_offset..*pack_offset + *entry_size as u64, resolve_buf)
                            .ok_or_else(|| Error::ConsumeResolveFailed(*pack_offset))?;
                        let (_header, decompressed_size, consumed) =
                            pack::data::Header::from_bytes(resolve_buf, *pack_offset);
                        decompress_all_at_once(&resolve_buf[consumed as usize..], decompressed_size as usize)?
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
            caches
                .lock()
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
    for Entry {
        pack_offset,
        kind,
        entry_len,
        ..
    } in &base_entries
    {
        let (is_borrowed, base_bytes) = decompressed_bytes_from_cache(pack_offset, entry_len)?;
        out.push((
            *pack_offset,
            compute_hash(kind.to_kind().expect("base object"), &base_bytes),
        ));
        possibly_return_to_cache(pack_offset, is_borrowed, base_bytes);
    }

    // find all deltas that match our bases, decompress them, apply them to the decompressed base, keep the hash
    // and finally store the fully decompressed delta as new base (if they have dependants of their own).
    // If there is nobody else using them, we could remove them, but that's expensive so let's just keep them around.
    out.shrink_to_fit();
    Ok(out)
}

fn decompress_all_at_once(b: &[u8], decompressed_len: usize) -> Result<Vec<u8>, Error> {
    let mut out = Vec::with_capacity(decompressed_len);
    zlib::Inflate::default()
        .once(&b, &mut io::Cursor::new(&mut out), true)
        .map_err(|err| Error::ConsumeZlibInflate(err, "Failed to decompress entry"))?;
    Ok(out)
}
