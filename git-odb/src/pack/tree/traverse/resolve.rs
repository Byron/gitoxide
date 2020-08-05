use crate::{
    pack::{self, data::EntrySlice, tree::traverse::Error},
    zlib,
};
use git_features::progress::Progress;
use git_object::HashKind;
use std::{cell::RefCell, collections::BTreeMap};

pub(crate) fn deltas<T, F, P, MBFN, BR, MCFN>(
    nodes: Vec<pack::tree::Node<T>>,
    (bytes_buf, progress): &mut (Vec<u8>, P),
    resolve: F,
    hash_kind: HashKind,
    modify_base: MBFN,
    modify_child: MCFN,
) -> Result<usize, Error>
where
    F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
    P: Progress,
    MBFN: for<'r> Fn(&'r mut T, &'r [u8], HashKind) -> BR,
    BR: Clone,
    MCFN: for<'r> Fn(&'r mut T, BR),
    T: Default,
{
    let mut decompressed_bytes_by_pack_offset = BTreeMap::new();
    let bytes_buf = RefCell::new(bytes_buf);
    let mut num_objects = 0;
    let decompress_from_resolver = |slice: EntrySlice| -> Result<Vec<u8>, Error> {
        let mut bytes_buf = bytes_buf.borrow_mut();
        bytes_buf.resize((slice.end - slice.start) as usize, 0);
        resolve(slice.clone(), &mut bytes_buf).ok_or_else(|| Error::ResolveFailed(slice.start))?;
        let entry = pack::data::Entry::from_bytes(&bytes_buf, slice.start);
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
            decompress_from_resolver(base.entry_slice())?
        } else {
            decompressed_bytes_by_pack_offset
                .remove(&base.offset())
                .expect("we store the resolved delta buffer when done")
        };

        let base_result = modify_base(&mut base.data, &base_bytes, hash_kind);
        num_objects += 1;
        for mut child in base.store_changes_then_into_child_iter() {
            let delta_bytes = decompress_from_resolver(child.entry_slice())?;
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

            decompressed_bytes_by_pack_offset.insert(child.offset(), fully_resolved_delta_bytes.to_owned());
            modify_child(&mut child.data, base_result.clone());
            nodes.push((level + 1, child));
        }
    }

    Ok(num_objects)
}

fn decompress_all_at_once(b: &[u8], decompressed_len: usize) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    out.resize(decompressed_len, 0);
    zlib::Inflate::default()
        .once(&b, &mut out, true)
        .map_err(|err| Error::ZlibInflate(err, "Failed to decompress entry"))?;
    Ok(out)
}
