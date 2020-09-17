use crate::pack::{self, index};
use git_features::progress::{self, Progress};
use git_object::{
    borrowed,
    bstr::{BString, ByteSlice},
    owned, SHA1_SIZE,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("index checksum mismatch: expected {expected}, got {actual}")]
    Mismatch { expected: owned::Id, actual: owned::Id },
    #[error("{kind} object {id} could not be decoded")]
    ObjectDecode {
        source: borrowed::Error,
        kind: git_object::Kind,
        id: owned::Id,
    },
    #[error("{kind} object {id} wasn't re-encoded without change, wanted\n{expected}\n\nGOT\n\n{actual}")]
    ObjectEncodeMismatch {
        kind: git_object::Kind,
        id: owned::Id,
        expected: BString,
        actual: BString,
    },
    #[error(transparent)]
    ObjectEncode(#[from] std::io::Error),
}

/// Various ways in which a pack and index can be verified
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Mode {
    /// Validate SHA1 and CRC32
    Sha1CRC32,
    /// Validate SHA1 and CRC32, and decode each non-Blob object
    Sha1CRC32Decode,
    /// Validate SHA1 and CRC32, and decode and encode each non-Blob object
    Sha1CRC32DecodeEncode,
}

/// Verify and validate the content of the index file
impl index::File {
    pub fn index_checksum(&self) -> owned::Id {
        owned::Id::from_20_bytes(&self.data[self.data.len() - SHA1_SIZE..])
    }

    pub fn pack_checksum(&self) -> owned::Id {
        let from = self.data.len() - SHA1_SIZE * 2;
        owned::Id::from_20_bytes(&self.data[from..from + SHA1_SIZE])
    }

    pub fn verify_checksum(&self, mut progress: impl Progress) -> Result<owned::Id, Error> {
        let data_len_without_trailer = self.data.len() - SHA1_SIZE;
        let actual = match crate::hash::bytes_of_file(&self.path, data_len_without_trailer, &mut progress) {
            Ok(id) => id,
            Err(_io_err) => {
                let start = std::time::Instant::now();
                let mut hasher = git_features::hash::Sha1::default();
                hasher.update(&self.data[..data_len_without_trailer]);
                progress.inc_by(data_len_without_trailer);
                progress.show_throughput(start);
                owned::Id::new_sha1(hasher.digest())
            }
        };

        let expected = self.index_checksum();
        if actual == expected {
            Ok(actual)
        } else {
            Err(Error::Mismatch { actual, expected })
        }
    }

    /// If `pack` is provided, it is expected (and validated to be) the pack belonging to this index.
    /// It will be used to validate internal integrity of the pack before checking each objects integrity
    /// is indeed as advertised via its SHA1 as stored in this index, as well as the CRC32 hash.
    /// redoing a lot of work across multiple objects.
    pub fn verify_integrity<C, P>(
        &self,
        pack: Option<(&pack::data::File, Mode, index::traverse::Algorithm)>,
        thread_limit: Option<usize>,
        progress: Option<P>,
        make_cache: impl Fn() -> C + Send + Sync,
    ) -> Result<
        (owned::Id, Option<index::traverse::Outcome>, Option<P>),
        index::traverse::Error<pack::index::verify::Error>,
    >
    where
        P: Progress,
        C: pack::cache::DecodeEntry,
    {
        let mut root = progress::DoOrDiscard::from(progress);
        match pack {
            Some((pack, mode, algorithm)) => self
                .traverse(
                    pack,
                    root.into_inner(),
                    || {
                        let mut encode_buf = Vec::with_capacity(2048);
                        move |kind, data, index_entry, progress| {
                            Self::verify_entry(mode, &mut encode_buf, kind, data, index_entry, progress)
                        }
                    },
                    make_cache,
                    index::traverse::Options {
                        algorithm,
                        thread_limit,
                        check: index::traverse::SafetyCheck::All,
                    },
                )
                .map(|(id, outcome, root)| (id, Some(outcome), root)),
            None => self
                .verify_checksum(root.add_child("Sha1 of index"))
                .map_err(Into::into)
                .map(|id| (id, None, root.into_inner())),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn verify_entry<P>(
        mode: Mode,
        encode_buf: &mut Vec<u8>,
        object_kind: git_object::Kind,
        buf: &[u8],
        index_entry: &index::Entry,
        progress: &mut P,
    ) -> Result<(), Error>
    where
        P: Progress,
    {
        if let Mode::Sha1CRC32Decode | Mode::Sha1CRC32DecodeEncode = mode {
            use git_object::Kind::*;
            match object_kind {
                Tree | Commit | Tag => {
                    let borrowed_object =
                        borrowed::Object::from_bytes(object_kind, buf).map_err(|err| Error::ObjectDecode {
                            source: err,
                            kind: object_kind,
                            id: index_entry.oid,
                        })?;
                    if let Mode::Sha1CRC32DecodeEncode = mode {
                        let object = owned::Object::from(borrowed_object);
                        encode_buf.clear();
                        object.write_to(&mut *encode_buf)?;
                        if encode_buf.as_slice() != buf {
                            let mut should_return_error = true;
                            if let git_object::Kind::Tree = object_kind {
                                if buf.as_bstr().find(b"100664").is_some() || buf.as_bstr().find(b"100640").is_some() {
                                    progress.info(format!("Tree object {} would be cleaned up during re-serialization, replacing mode '100664|100640' with '100644'", index_entry.oid));
                                    should_return_error = false
                                }
                            }
                            if should_return_error {
                                return Err(Error::ObjectEncodeMismatch {
                                    kind: object_kind,
                                    id: index_entry.oid,
                                    expected: buf.into(),
                                    actual: encode_buf.clone().into(),
                                });
                            }
                        }
                    }
                }
                Blob => {}
            };
        }
        Ok(())
    }
}
