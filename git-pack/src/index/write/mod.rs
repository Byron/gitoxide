use crate::{
    loose,
    tree::{traverse::Context, Tree},
};
use git_features::progress::{self, Progress};
use std::{convert::TryInto, io, sync::atomic::AtomicBool};

mod encode;
mod error;
pub use error::Error;

pub(crate) struct TreeEntry {
    pub id: git_hash::ObjectId,
    pub crc32: u32,
}

impl Default for TreeEntry {
    fn default() -> Self {
        TreeEntry {
            id: git_hash::ObjectId::null_sha1(),
            crc32: 0,
        }
    }
}

/// Information gathered while executing [`write_data_iter_to_stream()`][crate::index::File::write_data_iter_to_stream]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    /// The version of the verified index
    pub index_kind: crate::index::Version,
    /// The verified checksum of the verified index
    pub index_hash: git_hash::ObjectId,

    /// The hash of the '.pack' file, also found in its trailing bytes
    pub data_hash: git_hash::ObjectId,
    /// The amount of objects that were verified, always the amount of objects in the pack.
    pub num_objects: u32,
}

/// Various ways of writing an index file from pack entries
impl crate::index::File {
    /// Write information about `entries` as obtained from a pack data file into a pack index file via the `out` stream.
    /// The resolver produced by `make_resolver` must resolve pack entries from the same pack data file that produced the
    /// `entries` iterator.
    ///
    /// `kind` is the version of pack index to produce, use [`crate::index::Version::default()`] if in doubt.
    /// `tread_limit` is used for a parallel tree traversal for obtaining object hashes with optimal performance.
    /// `root_progress` is the top-level progress to stay informed about the progress of this potentially long-running
    /// computation.
    ///
    /// # Remarks
    ///
    /// * neither in-pack nor out-of-pack Ref Deltas are supported here, these must have been resolved beforehand.
    /// * `make_resolver()` will only be called after the iterator stopped returning elements and produces a function that
    /// provides all bytes belonging to a pack entry writing them to the given mutable output `Vec`.
    /// It should return `None` if the entry cannot be resolved from the pack that produced the `entries` iterator, causing
    /// the write operation to fail.
    pub fn write_data_iter_to_stream<F, F2>(
        kind: crate::index::Version,
        make_resolver: F,
        entries: impl Iterator<Item = Result<crate::data::input::Entry, crate::data::input::Error>>,
        thread_limit: Option<usize>,
        mut root_progress: impl Progress,
        out: impl io::Write,
        should_interrupt: &AtomicBool,
    ) -> Result<Outcome, Error>
    where
        F: FnOnce() -> io::Result<F2>,
        F2: for<'r> Fn(crate::data::EntryRange, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
    {
        if kind != crate::index::Version::default() {
            return Err(Error::Unsupported(kind));
        }
        let mut num_objects: usize = 0;
        let mut bytes_to_process = 0u64;
        let mut last_seen_trailer = None;
        let mut last_base_index = None;
        let anticipated_num_objects = entries.size_hint().0;
        let mut tree = Tree::with_capacity(anticipated_num_objects)?;
        let indexing_start = std::time::Instant::now();

        root_progress.init(Some(4), progress::steps());
        let mut objects_progress = root_progress.add_child("indexing");
        objects_progress.init(entries.size_hint().1, progress::count("objects"));
        let mut decompressed_progress = root_progress.add_child("decompressing");
        decompressed_progress.init(None, progress::bytes());
        let mut pack_entries_end: u64 = 0;

        for (eid, entry) in entries.enumerate() {
            let crate::data::input::Entry {
                header,
                pack_offset,
                crc32,
                header_size,
                compressed: _,
                compressed_size,
                decompressed_size,
                trailer,
            } = entry?;

            bytes_to_process += decompressed_size;
            decompressed_progress.inc_by(decompressed_size as usize);

            let entry_len = header_size as u64 + compressed_size;
            pack_entries_end = pack_offset + entry_len;

            let crc32 = crc32.expect("crc32 to be computed by the iterator. Caller assures correct configuration.");

            use crate::data::entry::Header::*;
            match header {
                Tree | Blob | Commit | Tag => {
                    last_base_index = Some(eid);
                    tree.add_root(
                        pack_offset,
                        TreeEntry {
                            id: git_hash::ObjectId::null_sha1(),
                            crc32,
                        },
                    )?;
                }
                RefDelta { .. } => return Err(Error::IteratorInvariantNoRefDelta),
                OfsDelta { base_distance } => {
                    let base_pack_offset =
                        crate::data::entry::Header::verified_base_pack_offset(pack_offset, base_distance).ok_or(
                            Error::IteratorInvariantBaseOffset {
                                pack_offset,
                                distance: base_distance,
                            },
                        )?;
                    tree.add_child(
                        base_pack_offset,
                        pack_offset,
                        TreeEntry {
                            id: git_hash::ObjectId::null_sha1(),
                            crc32,
                        },
                    )?;
                }
            };
            last_seen_trailer = trailer;
            num_objects += 1;
            objects_progress.inc();
        }
        if num_objects != anticipated_num_objects {
            objects_progress.info(format!(" {} objects, got {}", anticipated_num_objects, num_objects));
        }
        let num_objects: u32 = num_objects
            .try_into()
            .map_err(|_| Error::IteratorInvariantTooManyObjects(num_objects))?;
        last_base_index.ok_or(Error::IteratorInvariantBasesPresent)?;

        objects_progress.show_throughput(indexing_start);
        decompressed_progress.show_throughput(indexing_start);
        drop(objects_progress);
        drop(decompressed_progress);

        root_progress.inc();

        let resolver = make_resolver()?;
        let sorted_pack_offsets_by_oid = {
            let in_parallel_if_pack_is_big_enough = || bytes_to_process > 5_000_000;
            let mut items = tree.traverse(
                in_parallel_if_pack_is_big_enough,
                resolver,
                root_progress.add_child("Resolving"),
                root_progress.add_child("Decoding"),
                thread_limit,
                should_interrupt,
                pack_entries_end,
                || (),
                |data,
                 _progress,
                 Context {
                     entry,
                     decompressed: bytes,
                     ..
                 }| {
                    modify_base(data, entry, bytes, kind.hash());
                    Ok::<_, Error>(())
                },
            )?;
            root_progress.inc();

            {
                let _progress = root_progress.add_child("sorting by id");
                items.make_contiguous().sort_by_key(|e| e.data.id);
            }

            root_progress.inc();
            items
        };

        let pack_hash = last_seen_trailer.ok_or(Error::IteratorInvariantTrailer)?;
        let index_hash = encode::write_to(
            out,
            sorted_pack_offsets_by_oid,
            &pack_hash,
            kind,
            root_progress.add_child("writing index file"),
        )?;
        root_progress.show_throughput_with(
            indexing_start,
            num_objects as usize,
            progress::count("objects").expect("unit always set"),
        );
        Ok(Outcome {
            index_kind: kind,
            index_hash,
            data_hash: pack_hash,
            num_objects,
        })
    }
}

fn modify_base(
    entry: &mut crate::index::write::TreeEntry,
    pack_entry: &crate::data::Entry,
    decompressed: &[u8],
    hash: git_hash::Kind,
) {
    fn compute_hash(kind: git_object::Kind, bytes: &[u8], hash_kind: git_hash::Kind) -> git_hash::ObjectId {
        let mut write = git_features::hash::Write::new(io::sink(), hash_kind);
        loose::object::header::encode(kind, bytes.len() as u64, &mut write)
            .expect("write to sink and hash cannot fail");
        write.hash.update(bytes);
        git_hash::ObjectId::from(write.hash.digest())
    }

    let object_kind = pack_entry.header.as_kind().expect("base object as source of iteration");
    let id = compute_hash(object_kind, &decompressed, hash);
    entry.id = id;
}
