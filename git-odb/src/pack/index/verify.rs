use crate::pack::{self, index};
use git_features::progress::{self, Progress};
use git_object::{
    borrowed,
    bstr::{BString, ByteSlice},
    owned, SHA1_SIZE,
};
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Mismatch { expected: owned::Id, actual: owned::Id } {
            display("index checksum mismatch: expected {}, got {}", expected, actual)
        }
        ObjectDecode(err: borrowed::Error, kind: git_object::Kind, oid: owned::Id) {
            display("{} object {} could not be decoded", kind, oid)
            source(err)
        }
        ObjectEncodeMismatch(kind: git_object::Kind, oid: owned::Id, expected: BString, actual: BString) {
            display("{} object {} wasn't re-encoded without change, wanted\n{}\n\nGOT\n\n{}", kind, oid, expected, actual)
        }
        ObjectEncode(err: std::io::Error) {
            from()
        }
    }
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
        let throughput = index::util::TimeThroughput::new(self.data.len());
        let mut hasher = git_features::hash::Sha1::default();
        hasher.update(&self.data[..self.data.len() - SHA1_SIZE]);
        let actual = owned::Id::new_sha1(hasher.digest());
        progress.done(throughput);

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
    pub fn verify_integrity<P, C>(
        &self,
        pack: Option<(&pack::data::File, Mode, index::traverse::Algorithm)>,
        thread_limit: Option<usize>,
        progress: Option<P>,
        make_cache: impl Fn() -> C + Send + Sync,
    ) -> Result<(owned::Id, Option<index::traverse::Outcome>, Option<P>), index::traverse::Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        <<P as Progress>::SubProgress as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
    {
        let mut root = progress::DoOrDiscard::from(progress);

        let progress = root.add_child("Sha1 of index");
        let verify_self = move || self.verify_checksum(progress);

        match pack {
            None => verify_self()
                .map_err(Into::into)
                .map(|id| (id, None, root.into_inner())),
            Some((pack, mode, algorithm)) => self
                .traverse(
                    pack,
                    index::traverse::Context {
                        algorithm,
                        thread_limit,
                        check: index::traverse::SafetyCheck::All,
                    },
                    root.into_inner(),
                    || {
                        let mut encode_buf = Vec::with_capacity(2048);
                        move |kind, data, index_entry, stats, progress| {
                            Self::verify_entry(mode, &mut encode_buf, kind, data, index_entry, stats, progress)
                        }
                    },
                    make_cache,
                )
                .map(|(id, outcome, root)| (id, Some(outcome), root)),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn verify_entry<P>(
        mode: Mode,
        encode_buf: &mut Vec<u8>,
        object_kind: git_object::Kind,
        buf: &[u8],
        index_entry: &index::Entry,
        _stats: &pack::data::decode::Outcome,
        progress: &mut P,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        P: Progress,
    {
        if let Mode::Sha1CRC32Decode | Mode::Sha1CRC32DecodeEncode = mode {
            use git_object::Kind::*;
            match object_kind {
                Tree | Commit | Tag => {
                    let borrowed_object = borrowed::Object::from_bytes(object_kind, buf).map_err(|err| {
                        Box::new(Error::ObjectDecode(err, object_kind, index_entry.oid))
                            as Box<dyn std::error::Error + Send + Sync>
                    })?;
                    if let Mode::Sha1CRC32DecodeEncode = mode {
                        let object = owned::Object::from(borrowed_object);
                        encode_buf.clear();
                        object
                            .write_to(&mut *encode_buf)
                            .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)?;
                        if encode_buf.as_slice() != buf {
                            let mut should_return_error = true;
                            if let git_object::Kind::Tree = object_kind {
                                if buf.as_bstr().find(b"100664").is_some() || buf.as_bstr().find(b"100640").is_some() {
                                    progress.info(format!("Tree object {} would be cleaned up during re-serialization, replacing mode '100664|100640' with '100644'", index_entry.oid));
                                    should_return_error = false
                                }
                            }
                            if should_return_error {
                                return Err(Box::new(Error::ObjectEncodeMismatch(
                                    object_kind,
                                    index_entry.oid,
                                    buf.into(),
                                    encode_buf.clone().into(),
                                )));
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
