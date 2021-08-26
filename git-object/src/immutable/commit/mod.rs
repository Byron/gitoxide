use crate::{immutable::object, BStr, CommitRef};

mod decode;

///
pub mod iter;

impl<'a> CommitRef<'a> {
    /// Deserialize a commit from the given `data` bytes while avoiding most allocations.
    pub fn from_bytes(data: &'a [u8]) -> Result<CommitRef<'a>, object::decode::Error> {
        decode::commit(data)
            .map(|(_, t)| t)
            .map_err(object::decode::Error::from)
    }
    /// Return the `tree` fields hash digest.
    pub fn tree(&self) -> git_hash::ObjectId {
        git_hash::ObjectId::from_hex(self.tree).expect("prior validation of tree hash during parsing")
    }

    /// Returns an iterator of parent object ids
    pub fn parents(&self) -> impl Iterator<Item = git_hash::ObjectId> + '_ {
        self.parents
            .iter()
            .map(|hex_hash| git_hash::ObjectId::from_hex(hex_hash).expect("prior validation of hashes during parsing"))
    }

    /// Returns a convenient iterator over all extra headers.
    pub fn extra_headers(&self) -> crate::commit::ExtraHeaders<impl Iterator<Item = (&BStr, &BStr)>> {
        crate::commit::ExtraHeaders::new(self.extra_headers.iter().map(|(k, v)| (*k, v.as_ref())))
    }
}
