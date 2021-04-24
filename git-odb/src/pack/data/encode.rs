use git_hash::ObjectId;

/// An entry to be written to a file.
pub struct Entry {
    /// The hash of the object to write
    pub id: ObjectId,
    /// The kind of packed object
    pub object_kind: git_object::Kind,
    /// The kind of entry represented by `data`. It's used alongside with it to complete the pack entry
    /// at rest or in transit.
    pub entry_kind: entry::Kind,
    /// The size in bytes needed once `data` gets decompressed
    pub decompressed_size: usize,
    /// The compressed data right behind the header
    pub compressed_data: Vec<u8>,
}

///
pub mod entry {
    use crate::{data, pack::data::encode};
    use git_hash::ObjectId;

    /// The kind of pack entry to be written
    pub enum Kind {
        /// A complete base object
        Base,
        /// A delta against the object encountered `n` objects before (in this iteration)
        DeltaRef {
            /// Never 0, and 1 would mean the previous object acts as base object.
            nth_before: usize,
        },
        /// A delta against the given object as identified by its `ObjectId`.
        /// This is the case for thin packs only.
        /// Note that there is the option of the `ObjectId` being used to refer to an object within
        /// the same pack, but it's a discontinued practice which won't be encountered here.
        DeltaOid {
            /// The object serving as base for this delta
            id: ObjectId,
        },
    }

    /// The error returned by [`encode::Entry::new()`].
    #[allow(missing_docs)]
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("{0}")]
        ZlibDeflate(String),
    }

    impl encode::Entry {
        /// Create a new instance from the given `oid` and its corresponding git `obj`ect data.
        pub fn from_data(oid: impl Into<ObjectId>, obj: &data::Object<'_>) -> Result<Self, Error> {
            Ok(encode::Entry {
                id: oid.into(),
                object_kind: obj.kind,
                entry_kind: Kind::Base,
                decompressed_size: obj.data.len(),
                compressed_data: {
                    let mut out = crate::zlib::stream::DeflateWriter::new(Vec::new());
                    if let Err(err) = std::io::copy(&mut &obj.data[..], &mut out) {
                        match err.kind() {
                            std::io::ErrorKind::Other => return Err(Error::ZlibDeflate(err.to_string())),
                            err => unreachable!("Should never see other errors than zlib, but got {:?}", err,),
                        }
                    };
                    out.into_inner()
                },
            })
        }
    }
}

///
pub mod entries;
pub use entries::entries;
