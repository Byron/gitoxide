/// The error returned by [commit()](crate::commit()).
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {}

pub(super) mod function {
    use crate::commit::Error;

    /// Like [`tree()`](crate::tree()), but it takes only two commits to automatically compute the
    /// merge-bases among them.
    pub fn commit(our_commit: &gix_hash::oid, their_commit: &gix_hash::oid) -> Result<(), Error> {
        todo!()
    }
}
