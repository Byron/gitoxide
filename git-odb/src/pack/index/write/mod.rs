use crate::{
    loose, pack,
    pack::tree::{traverse::Context, Tree},
};
use git_features::{hash, progress::Progress};
use git_object::{owned, HashKind};
use std::{convert::Infallible, convert::TryInto, io};

mod encode;
mod error;
pub use error::Error;

pub struct TreeEntry {
    pub id: owned::Id,
    pub crc32: u32,
}

impl Default for TreeEntry {
    fn default() -> Self {
        TreeEntry {
            id: owned::Id::null(),
            crc32: 0,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub index_kind: pack::index::Kind,
    pub index_hash: owned::Id,
    pub pack_hash: owned::Id,
    pub num_objects: u32,
}

/// Various ways of writing an index file from pack entries
impl pack::index::File {
    /// Note that neither in-pack nor out-of-pack Ref Deltas are supported here, these must have been resolved beforehand.
    /// `make_resolver()`:  It will only be called after the iterator stopped returning elements and produces a function that
    /// provides all bytes belonging to an entry.
    pub fn write_data_iter_to_stream<F, F2, P>(
        kind: pack::index::Kind,
        make_resolver: F,
        entries: impl Iterator<Item = Result<pack::data::iter::Entry, pack::data::iter::Error>>,
        thread_limit: Option<usize>,
        mut root_progress: P,
        out: impl io::Write,
    ) -> Result<Outcome, Error>
    where
        F: FnOnce() -> io::Result<F2>,
        F2: for<'r> Fn(pack::data::EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
        P: Progress,
        <P as Progress>::SubProgress: Send,
    {
        if kind != pack::index::Kind::default() {
            return Err(Error::Unsupported(kind));
        }
        let mut num_objects: usize = 0;
        let mut bytes_to_process = 0u64;
        let mut last_seen_trailer = None;
        let mut last_base_index = None;
        let mut tree = Tree::with_capacity(entries.size_hint().0)?;
        let mut header_buf = [0u8; 16];
        let indexing_start = std::time::Instant::now();

        root_progress.init(Some(4), Some("steps"));
        let mut progress = root_progress.add_child("indexing");
        progress.init(entries.size_hint().1.map(|l| l as u32), Some("objects"));
        let mut pack_entries_end: u64 = 0;

        for (eid, entry) in entries.enumerate() {
            let pack::data::iter::Entry {
                header,
                pack_offset,
                header_size,
                compressed,
                decompressed: _,
                decompressed_size,
                trailer,
            } = entry?;

            let compressed_len = compressed.len();
            bytes_to_process += decompressed_size;
            let entry_len = header_size as usize + compressed_len;
            pack_entries_end = pack_offset + entry_len as u64;

            let crc32 = {
                let header_len = header.to_write(decompressed_size, header_buf.as_mut())?;
                let state = hash::crc32_update(0, &header_buf[..header_len]);
                hash::crc32_update(state, &compressed)
            };

            use pack::data::Header::*;
            match header {
                Blob | Tree | Commit | Tag => {
                    last_base_index = Some(eid);
                    tree.add_root(
                        pack_offset,
                        TreeEntry {
                            id: owned::Id::null(),
                            crc32,
                        },
                    )?;
                }
                RefDelta { .. } => return Err(Error::IteratorInvariantNoRefDelta),
                OfsDelta { base_distance } => {
                    let base_pack_offset = pack::data::Header::verified_base_pack_offset(pack_offset, base_distance)
                        .ok_or_else(|| Error::IteratorInvariantBaseOffset(pack_offset, base_distance))?;
                    tree.add_child(
                        base_pack_offset,
                        pack_offset,
                        TreeEntry {
                            id: owned::Id::null(),
                            crc32,
                        },
                    )?;
                }
            };
            last_seen_trailer = trailer;
            num_objects += 1;
            progress.inc();
        }
        let num_objects: u32 = num_objects
            .try_into()
            .map_err(|_| Error::IteratorInvariantTooManyObjects(num_objects))?;
        last_base_index.ok_or(Error::IteratorInvariantBasesPresent)?;
        progress.show_throughput(indexing_start, num_objects, "objects");
        drop(progress);

        root_progress.inc();

        let resolver = make_resolver()?;
        let sorted_pack_offsets_by_oid = {
            let mut items = tree.traverse(
                || bytes_to_process > 5_000_000,
                resolver,
                root_progress.add_child("Resolving"),
                thread_limit,
                pack_entries_end,
                || (),
                |data,
                 _progress,
                 Context {
                     entry,
                     decompressed: bytes,
                     ..
                 }| modify_base(data, entry, bytes, kind.hash()),
            )?;
            root_progress.inc();

            {
                let _progress = root_progress.add_child("sorting by id");
                items.sort_by_key(|e| e.data.id);
            }

            root_progress.inc();
            items
        };

        let pack_hash = last_seen_trailer.ok_or(Error::IteratorInvariantTrailer)?;
        let index_hash = encode::to_write(
            out,
            sorted_pack_offsets_by_oid,
            &pack_hash,
            kind,
            root_progress.add_child("writing index file"),
        )?;
        root_progress.show_throughput(indexing_start, num_objects, "objects");
        Ok(Outcome {
            index_kind: kind,
            index_hash,
            pack_hash,
            num_objects,
        })
    }
}

pub fn modify_base(
    entry: &mut pack::index::write::TreeEntry,
    pack_entry: &pack::data::Entry,
    decompressed: &[u8],
    hash: HashKind,
) -> Result<(), Infallible> {
    fn compute_hash(kind: git_object::Kind, bytes: &[u8], hash_kind: HashKind) -> owned::Id {
        let mut write = crate::hash::Write::new(io::sink(), hash_kind);
        loose::object::header::encode(kind, bytes.len() as u64, &mut write)
            .expect("write to sink and hash cannot fail");
        write.hash.update(bytes);
        owned::Id::from(write.hash.digest())
    }

    let object_kind = pack_entry.header.to_kind().expect("base object as source of iteration");
    let id = compute_hash(object_kind, &decompressed, hash);
    entry.id = id;
    Ok(())
}
