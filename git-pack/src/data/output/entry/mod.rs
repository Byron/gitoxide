use crate::{data, data::output, find};
use git_features::hash;
use git_hash::ObjectId;
use std::io::Write;

///
pub mod iter_from_counts;
pub use iter_from_counts::iter_from_counts;

/// An entry to be written to a file.
///
/// Some of these will be in-flight and in memory while waiting to be written. Memory requirements depend on the amount of compressed
/// data they hold.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The hash of the object to write
    pub id: ObjectId,
    /// The kind of packed object
    pub object_kind: git_object::Kind,
    /// The kind of entry represented by `data`. It's used alongside with it to complete the pack entry
    /// at rest or in transit.
    pub kind: Kind,
    /// The size in bytes needed once `data` gets decompressed
    pub decompressed_size: usize,
    /// The compressed data right behind the header
    pub compressed_data: Vec<u8>,
}

/// The kind of pack entry to be written
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// A complete base object
    Base,
    /// A delta against the object with the given index. It's always an index that was already encountered to refer only
    /// to object we have written already.
    DeltaRef {
        /// The absolute index to the object to serve as base. It's up to the writer to maintain enough state to allow producing
        /// a packed delta object from it.
        object_index: usize,
    },
    /// A delta against the given object as identified by its `ObjectId`.
    /// This is the case for thin packs only, i.e. those that are sent over the wire.
    /// Note that there is the option of the `ObjectId` being used to refer to an object within
    /// the same pack, but it's a discontinued practice which won't be encountered here.
    DeltaOid {
        /// The object serving as base for this delta
        id: ObjectId,
    },
}

/// The error returned by [`output::Entry::from_data()`].
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ZlibDeflate(#[from] std::io::Error),
    #[error("Entry expected to have hash {expected}, but it had {actual}")]
    PackToPackCopyCrc32Mismatch { actual: u32, expected: u32 },
}

impl output::Entry {
    /// Create an Entry from a previously counted object which is located in a pack. It's `entry` is provided here.
    /// The `version` specifies what kind of target `Entry` version the caller desires.
    pub fn from_pack_entry(
        entry: find::Entry<'_>,
        count: &output::Count,
        _potential_bases: &[output::Count],
        _allow_thin_pack: bool,
        target_version: crate::data::Version,
    ) -> Option<Result<Self, Error>> {
        if entry.version != target_version {
            return None;
        };

        let pack_entry = crate::data::Entry::from_bytes(entry.data, 0);
        if let Some(expected) = entry.crc32 {
            let actual = hash::crc32(entry.data);
            if actual != expected {
                return Some(Err(Error::PackToPackCopyCrc32Mismatch { actual, expected }));
            }
        }
        if pack_entry.header.is_base() {
            Some(Ok(output::Entry {
                id: count.id.to_owned(),
                object_kind: pack_entry.header.as_kind().expect("base object only"),
                kind: output::entry::Kind::Base,
                decompressed_size: pack_entry.decompressed_size as usize,
                compressed_data: entry.data[pack_entry.data_offset as usize..].to_owned(),
            }))
        } else {
            None
        }
    }

    /// Create a new instance from the given `oid` and its corresponding git `obj`ect data.
    pub fn from_data(count: &output::Count, obj: &data::Object<'_>) -> Result<Self, Error> {
        Ok(output::Entry {
            id: count.id.to_owned(),
            object_kind: obj.kind,
            kind: Kind::Base,
            decompressed_size: obj.data.len(),
            compressed_data: {
                let mut out = git_features::zlib::stream::deflate::Write::new(Vec::new());
                if let Err(err) = std::io::copy(&mut &*obj.data, &mut out) {
                    match err.kind() {
                        std::io::ErrorKind::Other => return Err(Error::ZlibDeflate(err)),
                        err => unreachable!("Should never see other errors than zlib, but got {:?}", err,),
                    }
                };
                out.flush()?;
                out.into_inner()
            },
        })
    }

    /// Transform ourselves into pack entry header of `version` which can be written into a pack.
    ///
    /// `index_to_pack(object_index) -> pack_offset` is a function to convert the base object's index into
    /// the input object array (if each object is numbered) to an offset into the pack.
    /// This information is known to the one calling the method.
    pub fn to_entry_header(
        &self,
        version: crate::data::Version,
        index_to_pack: impl FnOnce(usize) -> u64,
    ) -> crate::data::entry::Header {
        assert!(
            matches!(version, data::Version::V2),
            "we can only write V2 pack entries for now"
        );

        use Kind::*;
        match self.kind {
            Base => {
                use git_object::Kind::*;
                match self.object_kind {
                    Tree => data::entry::Header::Tree,
                    Blob => data::entry::Header::Blob,
                    Commit => data::entry::Header::Commit,
                    Tag => data::entry::Header::Tag,
                }
            }
            DeltaOid { id } => data::entry::Header::RefDelta { base_id: id.to_owned() },
            DeltaRef { object_index } => data::entry::Header::OfsDelta {
                base_distance: index_to_pack(object_index),
            },
        }
    }
}
