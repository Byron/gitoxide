use crate::{
    hash, loose, pack,
    pack::index::write::types::{EntrySlice, ObjectKind},
    pack::index::write::Error,
    zlib,
};
use git_features::progress::Progress;
use git_object::{owned, HashKind};
use std::{cell::RefCell, collections::BTreeMap, io};

pub(crate) fn apply_deltas<F, P>(
    nodes: Vec<pack::tree::Node<pack::index::write::types::TreeEntry>>,
    (bytes_buf, progress): &mut (Vec<u8>, P),
    resolve: F,
    hash_kind: HashKind,
) -> Result<usize, Error>
where
    F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
    P: Progress,
{
    let mut decompressed_bytes_by_pack_offset = BTreeMap::new();
    let bytes_buf = RefCell::new(bytes_buf);
    let mut num_objects = 0;
    let decompress_from_resolver = |pack_offset: u64, entry_size: usize| -> Result<Vec<u8>, Error> {
        let mut bytes_buf = bytes_buf.borrow_mut();
        bytes_buf.resize(entry_size, 0);
        resolve(pack_offset..pack_offset + entry_size as u64, &mut bytes_buf)
            .ok_or_else(|| Error::ConsumeResolveFailed(pack_offset))?;
        let entry = pack::data::Entry::from_bytes(&bytes_buf, pack_offset);
        decompress_all_at_once(
            &bytes_buf[entry.header_size() as usize..],
            entry.decompressed_size as usize,
        )
    };

    // Traverse the tree breadth first and loose the data produced for the base as it won't be needed anymore.
    progress.init(None, Some("objects"));

    // each node is a base, and its children always start out as deltas which become a base after applying them.
    // These will be pushed onto our stack until all are processed
    let root_level = 0;
    let mut nodes: Vec<_> = nodes.into_iter().map(|n| (root_level, n)).collect();
    while let Some((level, mut base)) = nodes.pop() {
        let base_bytes = if level == root_level {
            decompress_from_resolver(base.data.pack_offset, base.data.entry_len)?
        } else {
            decompressed_bytes_by_pack_offset
                .remove(&base.data.pack_offset)
                .expect("we store the resolved delta buffer when done")
        };
        let base_kind = base.data.kind.to_kind().expect("base object as source of iteration");
        let id = compute_hash(base_kind, &base_bytes, hash_kind);
        base.data.id = id;

        num_objects += 1;
        for mut child in base.store_changes_then_into_child_iter() {
            let delta_bytes = decompress_from_resolver(child.data.pack_offset, child.data.entry_len)?;
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

            decompressed_bytes_by_pack_offset.insert(child.data.pack_offset, fully_resolved_delta_bytes.to_owned());
            child.data.kind = ObjectKind::Base(base_kind);
            nodes.push((level + 1, child));
        }
    }

    Ok(num_objects)
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
        .once(&b, &mut out, true)
        .map_err(|err| Error::ConsumeZlibInflate(err, "Failed to decompress entry"))?;
    Ok(out)
}
