use crate::{
    pack::{self, data::EntrySlice, tree::traverse::Context, tree::traverse::Error},
    zlib,
};
use git_features::progress::{unit, Progress};
use std::{cell::RefCell, collections::BTreeMap};

pub(crate) fn deltas<T, F, P, MBFN, S, E>(
    nodes: Vec<pack::tree::Node<'_, T>>,
    (bytes_buf, ref mut progress, state): &mut (Vec<u8>, P, S),
    resolve: F,
    modify_base: MBFN,
) -> Result<(usize, u64), Error>
where
    F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
    P: Progress,
    MBFN: Fn(&mut T, &mut P, Context<'_, S>) -> Result<(), E>,
    T: Default,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut decompressed_bytes_by_pack_offset = BTreeMap::new();
    let bytes_buf = RefCell::new(bytes_buf);
    let mut num_objects = 0;
    let mut decompressed_bytes: u64 = 0;
    let decompress_from_resolver = |slice: EntrySlice| -> Result<(pack::data::Entry, u64, Vec<u8>), Error> {
        let mut bytes_buf = bytes_buf.borrow_mut();
        bytes_buf.resize((slice.end - slice.start) as usize, 0);
        resolve(slice.clone(), &mut bytes_buf).ok_or_else(|| Error::ResolveFailed(slice.start))?;
        let entry = pack::data::Entry::from_bytes(&bytes_buf, slice.start);
        let compressed = &bytes_buf[entry.header_size() as usize..];
        let decompressed_len = entry.decompressed_size as usize;
        Ok((entry, slice.end, decompress_all_at_once(compressed, decompressed_len)?))
    };

    // Traverse the tree breadth first and loose the data produced for the base as it won't be needed anymore.
    progress.init(
        None,
        Some(unit::dynamic(unit::Human::new(
            unit::human::Formatter::new(),
            "objects",
        ))),
    );

    // each node is a base, and its children always start out as deltas which become a base after applying them.
    // These will be pushed onto our stack until all are processed
    let root_level = 0;
    let mut nodes: Vec<_> = nodes.into_iter().map(|n| (root_level, n)).collect();
    while let Some((level, mut base)) = nodes.pop() {
        let (base_entry, entry_end, base_bytes) = if level == root_level {
            decompress_from_resolver(base.entry_slice())?
        } else {
            decompressed_bytes_by_pack_offset
                .remove(&base.offset())
                .expect("we store the resolved delta buffer when done")
        };

        modify_base(
            &mut base.data,
            progress,
            Context {
                entry: &base_entry,
                entry_end,
                decompressed: &base_bytes,
                state,
                level,
            },
        )
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)?;
        num_objects += 1;
        decompressed_bytes += base_bytes.len() as u64;
        progress.inc();
        for child in base.store_changes_then_into_child_iter() {
            let (mut child_entry, entry_end, delta_bytes) = decompress_from_resolver(child.entry_slice())?;
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

            // FIXME: this actually invalidates the "pack_offset()" computation, which is not obvious to consumers
            // at all
            child_entry.header = base_entry.header;
            decompressed_bytes_by_pack_offset.insert(
                child.offset(),
                (child_entry, entry_end, fully_resolved_delta_bytes.to_owned()),
            );
            nodes.push((level + 1, child));
        }
    }

    Ok((num_objects, decompressed_bytes))
}

fn decompress_all_at_once(b: &[u8], decompressed_len: usize) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    out.resize(decompressed_len, 0);
    zlib::Inflate::default()
        .once(&b, &mut out, true)
        .map_err(|err| Error::ZlibInflate(err, "Failed to decompress entry"))?;
    Ok(out)
}
