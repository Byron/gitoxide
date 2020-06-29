use crate::{pack, pack::index};
use git_object::SHA1_SIZE;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum ChecksumError {
        Mismatch { expected: git_object::Id, actual: git_object::Id } {
            display("index checksum mismatch: expected {}, got {}", expected, actual)
        }
        PackChecksum(err: pack::ChecksumError) {
            display("The pack of this index file failed to verify its checksums")
            from()
            cause(err)
        }
        PackDecode(err: pack::Error, id: git_object::Id, offset: u64) {
            display("Object {} at offset {} could not be decoded", id, offset)
            cause(err)
        }
        PackMismatch { expected: git_object::Id, actual: git_object::Id } {
            display("The packfiles checksum didn't match the index file checksum: expected {}, got {}", expected, actual)
        }
        PackObjectMismatch { expected: git_object::Id, actual: git_object::Id, offset: u64, kind: git_object::Kind} {
            display("The SHA1 of {} object at offset {} didn't match the checksum in the index file: expected {}, got {}", kind, offset, expected, actual)
        }
        Crc32Mismatch { expected: u32, actual: u32, offset: u64, kind: git_object::Kind} {
            display("The CRC32 of {} object at offset {} didn't match the checksum in the index file: expected {}, got {}", kind, offset, expected, actual)
        }
    }
}

/// Methods to verify and validate the content of the index file
impl index::File {
    pub fn checksum_of_index(&self) -> git_object::Id {
        git_object::Id::from_20_bytes(&self.data[self.data.len() - SHA1_SIZE..])
    }

    pub fn checksum_of_pack(&self) -> git_object::Id {
        let from = self.data.len() - SHA1_SIZE * 2;
        git_object::Id::from_20_bytes(&self.data[from..from + SHA1_SIZE])
    }

    /// If `pack` is provided, it is expected (and validated to be) the pack belonging to this index.
    /// It will be used to validate internal integrity of the pack before checking each objects integrity
    /// is indeed as advertised via its SHA1 as stored in this index, as well as the CRC32 hash.
    #[cfg(any(feature = "fast-sha1", feature = "minimal-sha1"))]
    pub fn verify_checksum_of_index(
        &self,
        pack: Option<&pack::File>,
    ) -> Result<git_object::Id, ChecksumError> {
        use crate::{
            pack::{cache, ResolvedBase},
            parallel::{self, in_parallel_if},
        };

        let verify_self = || {
            let mut hasher = crate::hash::Sha1::default();
            hasher.update(&self.data[..self.data.len() - SHA1_SIZE]);
            let actual = hasher.digest();

            let expected = self.checksum_of_index();
            if actual == expected {
                Ok(actual)
            } else {
                Err(ChecksumError::Mismatch { actual, expected })
            }
        };
        match pack {
            None => verify_self(),
            Some(pack) => {
                if self.checksum_of_pack() != pack.checksum() {
                    return Err(ChecksumError::PackMismatch {
                        actual: pack.checksum(),
                        expected: self.checksum_of_pack(),
                    });
                }
                let (pack_res, id) = parallel::join(|| pack.verify_checksum(), verify_self);
                pack_res?;
                let id = id?;

                let index_entries = {
                    let mut v: Vec<_> = self.iter().collect();
                    v.sort_by_key(|e| e.pack_offset);
                    v
                };

                struct Reducer;

                impl parallel::Reducer for Reducer {
                    type Input = Result<(), ChecksumError>;
                    type Output = ();
                    type Error = ChecksumError;

                    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error> {
                        input?;
                        Ok(())
                    }

                    fn finalize(&mut self) -> Result<Self::Output, Self::Error> {
                        Ok(())
                    }
                }

                const CHUNK_SIZE: usize = 1000;
                let there_are_enough_entries_to_process = || index_entries.len() > CHUNK_SIZE * 2;
                let input_chunks = index_entries
                    .chunks(CHUNK_SIZE.max(index_entries.len() / CHUNK_SIZE))
                    .into_iter();
                let state_per_thread =
                    || (cache::DecodeEntryLRU::default(), Vec::with_capacity(2048));
                in_parallel_if(
                    there_are_enough_entries_to_process,
                    input_chunks,
                    state_per_thread,
                    |entries: &[index::Entry], (cache, buf)| -> Result<(), ChecksumError> {
                        for index_entry in entries {
                            let pack_entry = pack.entry(index_entry.pack_offset);
                            let pack_entry_data_offset = pack_entry.data_offset;
                            let (object_kind, consumed_input) = pack
                                .decode_entry(
                                    pack_entry,
                                    buf,
                                    |id, _| {
                                        self.lookup_index(&id).map(|index| {
                                            ResolvedBase::InPack(
                                                pack.entry(self.pack_offset_at_index(index)),
                                            )
                                        })
                                    },
                                    cache,
                                )
                                .map_err(|e| {
                                    ChecksumError::PackDecode(
                                        e,
                                        index_entry.oid,
                                        index_entry.pack_offset,
                                    )
                                })?;

                            let mut header_buf = [0u8; 64];
                            let header_size = crate::loose::db::serde::write_header(
                                object_kind,
                                buf.len(),
                                &mut header_buf[..],
                            )
                            .expect("header buffer to be big enough");
                            let mut hasher = crate::hash::Sha1::default();
                            hasher.update(&header_buf[..header_size]);
                            hasher.update(buf.as_slice());
                            let actual_oid = hasher.digest();
                            if actual_oid != index_entry.oid {
                                return Err(ChecksumError::PackObjectMismatch {
                                    actual: actual_oid,
                                    expected: index_entry.oid.clone(),
                                    offset: index_entry.pack_offset,
                                    kind: object_kind,
                                });
                            }
                            if let Some(desired_crc32) = index_entry.crc32 {
                                let actual_crc32 = pack.entry_crc32(
                                    index_entry.pack_offset,
                                    (pack_entry_data_offset - index_entry.pack_offset) as usize
                                        + consumed_input,
                                );
                                if actual_crc32 != desired_crc32 {
                                    return Err(ChecksumError::Crc32Mismatch {
                                        actual: actual_crc32,
                                        expected: desired_crc32,
                                        offset: index_entry.pack_offset,
                                        kind: object_kind,
                                    });
                                }
                            }
                        }
                        Ok(())
                    },
                    Reducer,
                )?;

                Ok(id)
            }
        }
    }
}
