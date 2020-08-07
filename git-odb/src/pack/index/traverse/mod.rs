use crate::pack::{self, index, index::util::TimeThroughput};
use git_features::{
    parallel,
    progress::{self, Progress},
};
use git_object::owned;

mod indexed;
mod lookup;
mod reduce;
pub(crate) use reduce::Reducer;

mod error;
pub use error::Error;

mod types;
pub use types::{Algorithm, Options, Outcome, SafetyCheck};

/// Verify and validate the content of the index file
impl index::File {
    pub fn traverse<P, C, Processor, E>(
        &self,
        pack: &pack::data::File,
        progress: Option<P>,
        new_processor: impl Fn() -> Processor + Send + Sync,
        new_cache: impl Fn() -> C + Send + Sync,
        Options {
            algorithm,
            thread_limit,
            check,
        }: Options,
    ) -> Result<(owned::Id, Outcome, Option<P>), Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
        E: std::error::Error + Send + Sync + 'static,
        Processor: FnMut(
            git_object::Kind,
            &[u8],
            &index::Entry,
            &mut progress::DoOrDiscard<<<P as Progress>::SubProgress as Progress>::SubProgress>,
        ) -> Result<(), E>,
    {
        let mut root = progress::DoOrDiscard::from(progress);
        let id = self.possibly_verify(pack, check, &mut root)?;
        match algorithm {
            Algorithm::Lookup => self.traverse_with_lookup(check, thread_limit, new_processor, new_cache, root, pack),
            Algorithm::DeltaTreeLookup => {
                self.traverse_with_index_lookup(check, thread_limit, new_processor, root, pack)
            }
        }
        .map(|(stats, root)| (id, stats, root.into_inner()))
    }

    fn possibly_verify<P>(
        &self,
        pack: &pack::data::File,
        check: SafetyCheck,
        root: &mut progress::DoOrDiscard<P>,
    ) -> Result<owned::Id, Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
    {
        Ok(if check.file_checksum() {
            if self.pack_checksum() != pack.checksum() {
                return Err(Error::PackMismatch {
                    actual: pack.checksum(),
                    expected: self.pack_checksum(),
                });
            }
            let (pack_res, id) = parallel::join(
                {
                    let mut progress = root.add_child("Sha1 of pack");
                    move || {
                        let throughput = TimeThroughput::new(pack.data_len());
                        let res = pack.verify_checksum();
                        progress.done(throughput);
                        res
                    }
                },
                {
                    let progress = root.add_child("Sha1 of index");
                    move || self.verify_checksum(progress)
                },
            );
            pack_res?;
            id?
        } else {
            self.index_checksum()
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn decode_and_process_entry<C, P, E>(
        &self,
        check: SafetyCheck,
        pack: &pack::data::File,
        cache: &mut C,
        buf: &mut Vec<u8>,
        progress: &mut P,
        header_buf: &mut [u8; 64],
        index_entry: &pack::index::Entry,
        processor: &mut impl FnMut(git_object::Kind, &[u8], &index::Entry, &mut P) -> Result<(), E>,
    ) -> Result<pack::data::decode::Outcome, Error>
    where
        C: pack::cache::DecodeEntry,
        P: Progress,
        E: std::error::Error + Send + Sync + 'static,
    {
        let pack_entry = pack.entry(index_entry.pack_offset);
        let pack_entry_data_offset = pack_entry.data_offset;
        let entry_stats = pack
            .decode_entry(
                pack_entry,
                buf,
                |id, _| {
                    self.lookup(id).map(|index| {
                        pack::data::decode::ResolvedBase::InPack(pack.entry(self.pack_offset_at_index(index)))
                    })
                },
                cache,
            )
            .map_err(|e| Error::PackDecode(e, index_entry.oid, index_entry.pack_offset))?;
        let object_kind = entry_stats.kind;
        let header_size = (pack_entry_data_offset - index_entry.pack_offset) as usize;
        let entry_len = header_size + entry_stats.compressed_size;

        process_entry(
            check,
            object_kind,
            &buf,
            progress,
            header_buf,
            index_entry,
            || pack.entry_crc32(index_entry.pack_offset, entry_len),
            processor,
        )?;
        Ok(entry_stats)
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_entry<P, E>(
    check: SafetyCheck,
    object_kind: git_object::Kind,
    decompressed: &[u8],
    progress: &mut P,
    header_buf: &mut [u8; 64],
    index_entry: &pack::index::Entry,
    pack_entry_crc32: impl FnOnce() -> u32,
    processor: &mut impl FnMut(git_object::Kind, &[u8], &index::Entry, &mut P) -> Result<(), E>,
) -> Result<(), Error>
where
    P: Progress,
    E: std::error::Error + Send + Sync + 'static,
{
    if check.object_checksum() {
        let header_size =
            crate::loose::object::header::encode(object_kind, decompressed.len() as u64, &mut header_buf[..])
                .expect("header buffer to be big enough");
        let mut hasher = git_features::hash::Sha1::default();
        hasher.update(&header_buf[..header_size]);
        hasher.update(decompressed);

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
            let actual_crc32 = pack_entry_crc32();
            if actual_crc32 != desired_crc32 {
                return Err(Error::Crc32Mismatch {
                    actual: actual_crc32,
                    expected: desired_crc32,
                    offset: index_entry.pack_offset,
                    kind: object_kind,
                });
            }
        }
    }
    processor(object_kind, decompressed, &index_entry, progress)
        .map_err(|err| Error::Processor(Box::new(err) as Box<dyn std::error::Error + Send + Sync>))
}
