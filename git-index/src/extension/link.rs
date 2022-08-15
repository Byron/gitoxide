use crate::{
    extension::{Link, Signature},
    util::split_at_pos,
};

/// The signature of the link extension.
pub const SIGNATURE: Signature = *b"link";

/// Bitmaps to know which entries to delete or replace, even though details are still unknown.
#[derive(Clone)]
pub struct Bitmaps {
    /// A bitmap to signal which entries to delete, maybe.
    pub delete: git_bitmap::ewah::Vec,
    /// A bitmap to signal which entries to replace, maybe.
    pub replace: git_bitmap::ewah::Vec,
}

///
pub mod decode {

    /// The error returned when decoding link extensions.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("{0}")]
        Corrupt(&'static str),
        #[error("{kind} bitmap corrupt")]
        BitmapDecode {
            err: git_bitmap::ewah::decode::Error,
            kind: &'static str,
        },
    }
}

pub(crate) fn decode(data: &[u8], object_hash: git_hash::Kind) -> Result<Link, decode::Error> {
    let (id, data) = split_at_pos(data, object_hash.len_in_bytes())
        .ok_or(decode::Error::Corrupt(
            "link extension too short to read share index checksum",
        ))
        .map(|(id, d)| (git_hash::ObjectId::from(id), d))?;

    if data.is_empty() {
        return Ok(Link {
            shared_index_checksum: id,
            bitmaps: None,
        });
    }

    let (delete, data) =
        git_bitmap::ewah::decode(data).map_err(|err| decode::Error::BitmapDecode { kind: "delete", err })?;
    let (replace, data) =
        git_bitmap::ewah::decode(data).map_err(|err| decode::Error::BitmapDecode { kind: "replace", err })?;

    if !data.is_empty() {
        return Err(decode::Error::Corrupt("garbage trailing link extension"));
    }

    Ok(Link {
        shared_index_checksum: id,
        bitmaps: Some(Bitmaps { delete, replace }),
    })
}
