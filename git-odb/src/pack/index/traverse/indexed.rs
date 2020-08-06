use super::{Error, SafetyCheck};
use crate::{
    pack,
    pack::index::{self, util::index_entries_sorted_by_offset_ascending},
    pack::tree::traverse::Context,
};
use git_features::progress::Progress;

impl index::File {
    pub(crate) fn traverse_with_index_lookup<P, Processor>(
        &self,
        check: SafetyCheck,
        thread_limit: Option<usize>,
        new_processor: impl Fn() -> Processor + Send + Sync,
        mut root: P,
        pack: &pack::data::File,
    ) -> Result<(index::traverse::Outcome, P), Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        Processor: FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &mut <<P as Progress>::SubProgress as Progress>::SubProgress,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
    {
        let sorted_entries = index_entries_sorted_by_offset_ascending(self, root.add_child("collecting sorted index"));
        let tree = pack::tree::Tree::from_offsets_in_pack(
            sorted_entries.into_iter().map(|e| EntryWithDefault::from(e)),
            |e| e.index_entry.pack_offset,
            pack.path(),
            root.add_child("indexing"),
            |id| self.lookup(id).map(|idx| self.pack_offset_at_index(idx)),
        )?;
        let there_are_enough_objects = || self.num_objects > 10_000;
        let outcome = digest_statistics(tree.traverse(
            there_are_enough_objects,
            |slice, out| pack.entry_slice(slice).map(|entry| out.copy_from_slice(entry)),
            root.add_child("Resolving"),
            thread_limit,
            pack.pack_end() as u64,
            || (new_processor(), [0u8; 64]),
            |data,
             progress,
             Context {
                 entry: pack_entry,
                 entry_end,
                 decompressed: bytes,
                 state: (ref mut processor, ref mut header_buf),
                 level,
             }| {
                let object_kind = pack_entry.header.to_kind().expect("non-delta object");
                data.level = level;
                data.decompressed_size = pack_entry.decompressed_size;
                data.header_size = pack_entry.header_size() as u16;
                data.object_kind = object_kind;
                data.compressed_size = entry_end - pack_entry.data_offset;
                data.object_size = bytes.len() as u64;
                let result = pack::index::traverse::process_entry(
                    check,
                    object_kind,
                    bytes,
                    progress,
                    header_buf,
                    &data.index_entry,
                    || {
                        // debug_assert_eq!(&data.index_entry.pack_offset, &pack_entry.pack_offset()); // TODO: Fix this
                        git_features::hash::crc32(
                            pack.entry_slice(data.index_entry.pack_offset..entry_end)
                                .expect("slice pointing into the pack (by now data is verified)"),
                        )
                    },
                    processor,
                );
                match result {
                    Err(err @ Error::PackDecode(_, _, _)) if !check.fatal_decode_error() => {
                        progress.info(format!("Ignoring decode error: {}", err));
                        Ok(())
                    }
                    res => res,
                }
            },
        )?);
        Ok((outcome, root))
    }
}

pub struct EntryWithDefault {
    index_entry: pack::index::Entry,
    object_kind: git_object::Kind,
    object_size: u64,
    decompressed_size: u64,
    compressed_size: u64,
    header_size: u16,
    level: u16,
}

impl Default for EntryWithDefault {
    fn default() -> Self {
        EntryWithDefault {
            index_entry: pack::index::Entry {
                pack_offset: 0,
                crc32: None,
                oid: git_object::owned::Id::null(),
            },
            level: 0,
            object_kind: git_object::Kind::Tree,
            object_size: 0,
            decompressed_size: 0,
            compressed_size: 0,
            header_size: 0,
        }
    }
}

impl From<pack::index::Entry> for EntryWithDefault {
    fn from(index_entry: pack::index::Entry) -> Self {
        EntryWithDefault {
            index_entry,
            level: 0,
            object_kind: git_object::Kind::Tree,
            object_size: 0,
            decompressed_size: 0,
            compressed_size: 0,
            header_size: 0,
        }
    }
}

fn digest_statistics(items: Vec<pack::tree::Item<EntryWithDefault>>) -> index::traverse::Outcome {
    unimplemented!();
}
