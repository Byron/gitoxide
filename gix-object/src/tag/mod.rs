use winnow::prelude::*;

use crate::TagRef;

mod decode;

///
#[allow(clippy::empty_docs)]
pub mod write;

///
#[allow(clippy::empty_docs)]
pub mod ref_iter;

impl<'a> TagRef<'a> {
    /// Deserialize a tag from `data`.
    pub fn from_bytes(mut data: &'a [u8]) -> Result<TagRef<'a>, crate::decode::Error> {
        let input = &mut data;
        match decode::git_tag.parse_next(input) {
            Ok(tag) => Ok(tag),
            Err(err) => Err(crate::decode::Error::with_err(err, input)),
        }
    }
    /// The object this tag points to as `Id`.
    pub fn target(&self) -> gix_hash::ObjectId {
        gix_hash::ObjectId::from_hex(self.target).expect("prior validation")
    }
}
