use crate::TagRef;

mod decode;

///
pub mod write;

///
pub mod ref_iter;

impl<'a> TagRef<'a> {
    /// Deserialize a tag from `data`.
    pub fn from_bytes(data: &'a [u8]) -> Result<TagRef<'a>, crate::decode::Error> {
        decode::git_tag(data)
            .map(|(_, t)| t)
            .map_err(crate::decode::Error::from)
    }
    /// The object this tag points to as `Id`.
    pub fn target(&self) -> gix_hash::ObjectId {
        gix_hash::ObjectId::from_hex(self.target).expect("prior validation")
    }
}
