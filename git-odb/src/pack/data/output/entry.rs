use crate::{data, pack::data::output};
use git_hash::ObjectId;
use std::io::Write;

/// The kind of pack entry to be written
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
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

/// The error returned by [`output::Entry::from_data()`].
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ZlibDeflate(#[from] std::io::Error),
}

impl output::Entry {
    /// Create a new instance from the given `oid` and its corresponding git `obj`ect data.
    pub fn from_data(oid: impl Into<ObjectId>, obj: &data::Object<'_>) -> Result<Self, Error> {
        Ok(output::Entry {
            id: oid.into(),
            object_kind: obj.kind,
            entry_kind: Kind::Base,
            decompressed_size: obj.data.len(),
            compressed_data: {
                let mut out = crate::zlib::stream::deflate::Write::new(Vec::new());
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
}
