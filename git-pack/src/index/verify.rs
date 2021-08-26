use std::sync::{atomic::AtomicBool, Arc};

use git_features::progress::{self, Progress};
use git_hash::SIZE_OF_SHA1_DIGEST as SHA1_SIZE;
use git_object::{
    bstr::{BString, ByteSlice},
    immutable::object,
};

use crate::index;

/// Returned by [`index::File::verify_checksum()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("index checksum mismatch: expected {expected}, got {actual}")]
    Mismatch {
        expected: git_hash::ObjectId,
        actual: git_hash::ObjectId,
    },
    #[error("{kind} object {id} could not be decoded")]
    ObjectDecode {
        source: object::decode::Error,
        kind: git_object::Kind,
        id: git_hash::ObjectId,
    },
    #[error("{kind} object {id} wasn't re-encoded without change, wanted\n{expected}\n\nGOT\n\n{actual}")]
    ObjectEncodeMismatch {
        kind: git_object::Kind,
        id: git_hash::ObjectId,
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
    Sha1Crc32,
    /// Validate SHA1 and CRC32, and decode each non-Blob object.
    /// Each object should be valid, i.e. be decodable.
    Sha1Crc32Decode,
    /// Validate SHA1 and CRC32, and decode and encode each non-Blob object.
    /// Each object should yield exactly the same hash when re-encoded.
    Sha1Crc32DecodeEncode,
}

/// Verify and validate the content of the index file
impl index::File {
    /// Returns the trailing hash stored at the end of this index file.
    ///
    /// It's a hash over all bytes of the index.
    pub fn index_checksum(&self) -> git_hash::ObjectId {
        git_hash::ObjectId::from_20_bytes(&self.data[self.data.len() - SHA1_SIZE..])
    }

    /// Returns the hash of the pack data file that this index file corresponds to.
    ///
    /// It should [`crate::data::File::checksum()`] of the corresponding pack data file.
    pub fn pack_checksum(&self) -> git_hash::ObjectId {
        let from = self.data.len() - SHA1_SIZE * 2;
        git_hash::ObjectId::from_20_bytes(&self.data[from..from + SHA1_SIZE])
    }

    /// Validate that our [`index_checksum()`][index::File::index_checksum()] matches the actual contents
    /// of this index file, and return it if it does.
    pub fn verify_checksum(
        &self,
        mut progress: impl Progress,
        should_interrupt: &AtomicBool,
    ) -> Result<git_hash::ObjectId, Error> {
        let data_len_without_trailer = self.data.len() - SHA1_SIZE;
        let actual = match git_features::hash::bytes_of_file(
            &self.path,
            data_len_without_trailer,
            git_hash::Kind::Sha1,
            &mut progress,
            should_interrupt,
        ) {
            Ok(id) => id,
            Err(_io_err) => {
                let start = std::time::Instant::now();
                let mut hasher = git_features::hash::Sha1::default();
                hasher.update(&self.data[..data_len_without_trailer]);
                progress.inc_by(data_len_without_trailer);
                progress.show_throughput(start);
                git_hash::ObjectId::new_sha1(hasher.digest())
            }
        };

        let expected = self.index_checksum();
        if actual == expected {
            Ok(actual)
        } else {
            Err(Error::Mismatch { actual, expected })
        }
    }

    /// The most thorough validation of integrity of both index file and the corresponding pack data file, if provided.
    /// Returns the checksum of the index file, the traversal outcome and the given progress if the integrity check is successful.
    ///
    /// If `pack` is provided, it is expected (and validated to be) the pack belonging to this index.
    /// It will be used to validate internal integrity of the pack before checking each objects integrity
    /// is indeed as advertised via its SHA1 as stored in this index, as well as the CRC32 hash.
    /// The last member of the Option is a function returning an implementation of [`crate::cache::DecodeEntry`] to be used if
    /// the [`index::traverse::Algorithm`] is `Lookup`.
    /// To set this to `None`, use `None::<(_, _, _, fn() -> crate::cache::Never)>`.
    ///
    /// The `thread_limit` optionally specifies the amount of threads to be used for the [pack traversal][index::File::traverse()].
    /// `make_cache` is only used in case a `pack` is specified, use existing implementations in the [`crate::cache`] module.
    ///
    /// # Tradeoffs
    ///
    /// The given `progress` is inevitably consumed if there is an error, which is a tradeoff chosen to easily allow using `?` in the
    /// error case.
    pub fn verify_integrity<C, P>(
        &self,
        pack: Option<(
            &crate::data::File,
            Mode,
            index::traverse::Algorithm,
            impl Fn() -> C + Send + Sync,
        )>,
        thread_limit: Option<usize>,
        progress: Option<P>,
        should_interrupt: Arc<AtomicBool>,
    ) -> Result<
        (git_hash::ObjectId, Option<index::traverse::Outcome>, Option<P>),
        index::traverse::Error<crate::index::verify::Error>,
    >
    where
        P: Progress,
        C: crate::cache::DecodeEntry,
    {
        let mut root = progress::DoOrDiscard::from(progress);
        match pack {
            Some((pack, mode, algorithm, make_cache)) => self
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
                        should_interrupt,
                    },
                )
                .map(|(id, outcome, root)| (id, Some(outcome), root)),
            None => self
                .verify_checksum(root.add_child("Sha1 of index"), &should_interrupt)
                .map_err(Into::into)
                .map(|id| (id, None, root.into_inner())),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn verify_entry<P>(
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
        if let Mode::Sha1Crc32Decode | Mode::Sha1Crc32DecodeEncode = mode {
            use git_object::Kind::*;
            match object_kind {
                Tree | Commit | Tag => {
                    let borrowed_object =
                        git_object::ObjectRef::from_bytes(object_kind, buf).map_err(|err| Error::ObjectDecode {
                            source: err,
                            kind: object_kind,
                            id: index_entry.oid,
                        })?;
                    if let Mode::Sha1Crc32DecodeEncode = mode {
                        let object = git_object::Object::from(borrowed_object);
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
