use super::{Error, Mode, Outcome, Reducer};
use crate::pack::{self, data::decode, index, index::verify::util};
use git_features::{
    parallel::in_parallel_if,
    progress::{self, Progress},
};
use git_object::{borrowed, bstr::ByteSlice, owned};

/// Verify and validate the content of the index file
impl index::File {
    pub(crate) fn inner_verify_with_lookup<P, C>(
        &self,
        thread_limit: Option<usize>,
        mode: Mode,
        make_cache: impl Fn() -> C + Send + Sync,
        mut root: progress::DoOrDiscard<P>,
        pack: &pack::data::File,
    ) -> Result<Outcome, Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
    {
        let index_entries =
            util::index_entries_sorted_by_offset_ascending(self, root.add_child("collecting sorted index"));

        const CHUNK_SIZE: usize = 1000;
        let there_are_enough_entries_to_process = || index_entries.len() > CHUNK_SIZE * 2;
        let input_chunks = index_entries.chunks(CHUNK_SIZE.max(index_entries.len() / CHUNK_SIZE));
        let reduce_progress = std::sync::Mutex::new({
            let mut p = root.add_child("Checking");
            p.init(Some(self.num_objects()), Some("objects"));
            p
        });
        let state_per_thread = |index| {
            (
                make_cache(),
                Vec::with_capacity(2048), // decode buffer
                Vec::with_capacity(2048), // re-encode buffer
                reduce_progress.lock().unwrap().add_child(format!("thread {}", index)), // per thread progress
            )
        };

        in_parallel_if(
            there_are_enough_entries_to_process,
            input_chunks,
            thread_limit,
            state_per_thread,
            |entries: &[index::Entry], (cache, buf, encode_buf, progress)| -> Result<Vec<decode::Outcome>, Error> {
                progress.init(Some(entries.len() as u32), Some("entries"));
                let mut stats = Vec::with_capacity(entries.len());
                let mut header_buf = [0u8; 64];
                for (idx, index_entry) in entries.iter().enumerate() {
                    stats.push(self.process_entry(
                        mode,
                        pack,
                        cache,
                        buf,
                        encode_buf,
                        progress,
                        &mut header_buf,
                        index_entry,
                    )?);
                    progress.set(idx as u32);
                }
                Ok(stats)
            },
            Reducer::from_progress(&reduce_progress, pack.data_len()),
        )
    }

    pub(crate) fn process_entry<C>(
        &self,
        mode: Mode,
        pack: &pack::data::File,
        cache: &mut C,
        buf: &mut Vec<u8>,
        encode_buf: &mut Vec<u8>,
        progress: &mut impl Progress,
        header_buf: &mut [u8; 64],
        index_entry: &pack::index::Entry,
    ) -> Result<decode::Outcome, Error>
    where
        C: pack::cache::DecodeEntry,
    {
        let pack_entry = pack.entry(index_entry.pack_offset);
        let pack_entry_data_offset = pack_entry.data_offset;
        let entry_stats = pack
            .decode_entry(
                pack_entry,
                buf,
                |id, _| {
                    self.lookup_index(id).map(|index| {
                        pack::data::decode::ResolvedBase::InPack(pack.entry(self.pack_offset_at_index(index)))
                    })
                },
                cache,
            )
            .map_err(|e| Error::PackDecode(e, index_entry.oid, index_entry.pack_offset))?;
        let object_kind = entry_stats.kind;
        let consumed_input = entry_stats.compressed_size;

        let header_size = crate::loose::object::header::encode(object_kind, buf.len(), &mut header_buf[..])
            .expect("header buffer to be big enough");
        let mut hasher = git_features::hash::Sha1::default();
        hasher.update(&header_buf[..header_size]);
        hasher.update(buf.as_slice());

        let actual_oid = owned::Id::new_sha1(hasher.digest());
        if actual_oid != index_entry.oid {
            return Err(Error::PackObjectMismatch {
                actual: actual_oid,
                expected: index_entry.oid,
                offset: index_entry.pack_offset,
                kind: object_kind,
            });
        }
        if let Some(desired_crc32) = index_entry.crc32 {
            let header_size = (pack_entry_data_offset - index_entry.pack_offset) as usize;
            let actual_crc32 = pack.entry_crc32(index_entry.pack_offset, header_size + consumed_input);
            if actual_crc32 != desired_crc32 {
                return Err(Error::Crc32Mismatch {
                    actual: actual_crc32,
                    expected: desired_crc32,
                    offset: index_entry.pack_offset,
                    kind: object_kind,
                });
            }
        }
        if let Mode::Sha1CRC32Decode | Mode::Sha1CRC32DecodeEncode = mode {
            use git_object::Kind::*;
            match object_kind {
                Tree | Commit | Tag => {
                    let obj = borrowed::Object::from_bytes(object_kind, buf.as_slice())
                        .map_err(|err| Error::ObjectDecode(err, object_kind, index_entry.oid))?;
                    if let Mode::Sha1CRC32DecodeEncode = mode {
                        let object = owned::Object::from(obj);
                        encode_buf.clear();
                        object.write_to(&mut *encode_buf)?;
                        if encode_buf != buf {
                            let mut should_return_error = true;
                            if let git_object::Kind::Tree = object_kind {
                                if buf.as_slice().as_bstr().find(b"100664").is_some()
                                    || buf.as_slice().as_bstr().find(b"100640").is_some()
                                {
                                    progress.info(format!("Tree object {} would be cleaned up during re-serialization, replacing mode '100664|100640' with '100644'", index_entry.oid));
                                    should_return_error = false
                                }
                            }
                            if should_return_error {
                                return Err(Error::ObjectEncodeMismatch(
                                    object_kind,
                                    index_entry.oid,
                                    buf.clone().into(),
                                    encode_buf.clone().into(),
                                ));
                            }
                        }
                    }
                }
                Blob => {}
            };
        }
        Ok(entry_stats)
    }
}
