use crate::{
    hash, loose, pack,
    pack::index::write::types::{Cache, EntrySlice, Mode, ObjectKind},
    pack::index::write::Error,
    zlib,
};
use git_features::progress::Progress;
use git_object::{owned, HashKind};
use std::{cell::RefCell, io};

pub(crate) fn apply_deltas<F, P>(
    mut nodes: Vec<pack::tree::Node<pack::index::write::types::TreeEntry>>,
    (bytes_buf, progress): &mut (Vec<u8>, P),
    mode: &Mode<F>,
    hash_kind: HashKind,
) -> Result<usize, Error>
where
    F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
    P: Progress,
{
    let bytes_buf = RefCell::new(bytes_buf);
    let mut num_objects = 0;
    let decompress_from_cache = |cache: Cache, pack_offset: u64, entry_size: usize| -> Result<Vec<u8>, Error> {
        Ok(match cache {
            Cache::Unset => {
                let mut bytes_buf = bytes_buf.borrow_mut();
                bytes_buf.resize(entry_size, 0);
                match mode {
                    Mode::ResolveDeltas(r) | Mode::ResolveBases(r) | Mode::ResolveBasesAndDeltas(r) => {
                        r(pack_offset..pack_offset + entry_size as u64, &mut bytes_buf)
                            .ok_or_else(|| Error::ConsumeResolveFailed(pack_offset))?;
                        let entry = pack::data::Entry::from_bytes(&bytes_buf, pack_offset);
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
            Cache::Compressed(bytes, decompressed_len) => decompress_all_at_once(&bytes, decompressed_len)?,
            Cache::Decompressed(bytes) => bytes,
        })
    };

    // Traverse the tree breadth first and loose the data produced for the base as it won't be needed anymore.
    progress.init(None, Some("objects"));

    // each node is a base, and its children always start out as deltas which become a base after applying them.
    // These will be pushed onto our stack until all are processed
    while let Some(mut base) = nodes.pop() {
        let base_bytes = decompress_from_cache(extract_cache(&mut base), base.data.pack_offset, base.data.entry_len)?;
        let base_kind = base.data.kind.to_kind().expect("base object as source of iteration");
        let id = compute_hash(base_kind, &base_bytes, hash_kind);
        num_objects += 1;

        base.data.id = Some(id);
        for mut child in base.into_child_iter() {
            let delta_bytes =
                decompress_from_cache(extract_cache(&mut child), child.data.pack_offset, child.data.entry_len)?;
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

            child.data.cache = Cache::Decompressed(fully_resolved_delta_bytes.to_owned());
            child.data.kind = ObjectKind::Base(base_kind);
            nodes.push(child);
        }
    }

    Ok(num_objects)
}

fn extract_cache(node: &mut pack::tree::Node<pack::index::write::types::TreeEntry>) -> Cache {
    std::mem::replace(&mut node.data.cache, Cache::Unset)
}

fn compute_hash(kind: git_object::Kind, bytes: &[u8], hash_kind: HashKind) -> owned::Id {
    let mut write = hash::Write::new(io::sink(), hash_kind);
    loose::object::header::encode(kind, bytes.len() as u64, &mut write).expect("write to sink and hash cannot fail");
    write.hash.update(bytes);
    owned::Id::from(write.hash.digest())
}

fn decompress_all_at_once(b: &[u8], decompressed_len: usize) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    out.resize(decompressed_len, 0);
    zlib::Inflate::default()
        .once(&b, &mut io::Cursor::new(&mut out), true)
        .map_err(|err| Error::ConsumeZlibInflate(err, "Failed to decompress entry"))?;
    Ok(out)
}
