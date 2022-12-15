use std::{cell::RefCell, collections::BTreeMap, sync::atomic::Ordering};

use git_features::{
    progress::{unit, Progress},
    zlib,
};

use crate::{
    cache::delta::{
        traverse::{
            util::{ItemSliceSend, Node},
            Context, Error,
        },
        Item,
    },
    data::EntryRange,
};

pub(crate) fn deltas<T, F, P, MBFN, S, E>(
    object_counter: Option<git_features::progress::StepShared>,
    size_counter: Option<git_features::progress::StepShared>,
    node: &mut crate::cache::delta::Item<T>,
    (bytes_buf, ref mut progress, state, resolve, modify_base, child_items): &mut (
        Vec<u8>,
        P,
        S,
        F,
        MBFN,
        ItemSliceSend<Item<T>>,
    ),
    hash_len: usize,
) -> Result<(), Error>
where
    T: Send,
    F: for<'r> Fn(EntryRange, &'r mut Vec<u8>) -> Option<()>,
    P: Progress,
    MBFN: Fn(&mut T, &mut P, Context<'_, S>) -> Result<(), E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut decompressed_bytes_by_pack_offset = BTreeMap::new();
    let bytes_buf = RefCell::new(bytes_buf);
    let decompress_from_resolver = |slice: EntryRange| -> Result<(crate::data::Entry, u64, Vec<u8>), Error> {
        let mut bytes_buf = bytes_buf.borrow_mut();
        bytes_buf.resize((slice.end - slice.start) as usize, 0);
        resolve(slice.clone(), &mut bytes_buf).ok_or(Error::ResolveFailed {
            pack_offset: slice.start,
        })?;
        let entry = crate::data::Entry::from_bytes(&bytes_buf, slice.start, hash_len);
        let compressed = &bytes_buf[entry.header_size()..];
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
    let mut nodes: Vec<_> = vec![(
        root_level,
        Node {
            item: node,
            child_items: child_items.0,
        },
    )];
    while let Some((level, mut base)) = nodes.pop() {
        let (base_entry, entry_end, base_bytes) = if level == root_level {
            decompress_from_resolver(base.entry_slice())?
        } else {
            decompressed_bytes_by_pack_offset
                .remove(&base.offset())
                .expect("we store the resolved delta buffer when done")
        };

        // anything done here must be repeated further down for leaf-nodes.
        // This way we avoid retaining their decompressed memory longer than needed (they have no children,
        // thus their memory can be released right away, using 18% less peak memory on the linux kernel).
        {
            modify_base(
                base.data(),
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
            object_counter.as_ref().map(|c| c.fetch_add(1, Ordering::SeqCst));
            size_counter
                .as_ref()
                .map(|c| c.fetch_add(base_bytes.len(), Ordering::SeqCst));
        }

        for mut child in base.into_child_iter() {
            let (mut child_entry, entry_end, delta_bytes) = decompress_from_resolver(child.entry_slice())?;
            let (base_size, consumed) = crate::data::delta::decode_header_size(&delta_bytes);
            let mut header_ofs = consumed;
            assert_eq!(
                base_bytes.len(),
                base_size as usize,
                "recorded base size in delta does not match"
            );
            let (result_size, consumed) = crate::data::delta::decode_header_size(&delta_bytes[consumed..]);
            header_ofs += consumed;

            let mut fully_resolved_delta_bytes = bytes_buf.borrow_mut();
            fully_resolved_delta_bytes.resize(result_size as usize, 0);
            crate::data::delta::apply(&base_bytes, &mut fully_resolved_delta_bytes, &delta_bytes[header_ofs..]);

            // FIXME: this actually invalidates the "pack_offset()" computation, which is not obvious to consumers
            //        at all
            child_entry.header = base_entry.header; // assign the actual object type, instead of 'delta'
            if child.has_children() {
                decompressed_bytes_by_pack_offset.insert(
                    child.offset(),
                    (child_entry, entry_end, fully_resolved_delta_bytes.to_owned()),
                );
                nodes.push((level + 1, child));
            } else {
                modify_base(
                    child.data(),
                    progress,
                    Context {
                        entry: &child_entry,
                        entry_end,
                        decompressed: &fully_resolved_delta_bytes,
                        state,
                        level: level + 1,
                    },
                )
                .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)?;
                object_counter.as_ref().map(|c| c.fetch_add(1, Ordering::SeqCst));
                size_counter
                    .as_ref()
                    .map(|c| c.fetch_add(base_bytes.len(), Ordering::SeqCst));
            }
        }
    }

    Ok(())
}

fn decompress_all_at_once(b: &[u8], decompressed_len: usize) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    out.resize(decompressed_len, 0);
    zlib::Inflate::default()
        .once(b, &mut out)
        .map_err(|err| Error::ZlibInflate {
            source: err,
            message: "Failed to decompress entry",
        })?;
    Ok(out)
}
