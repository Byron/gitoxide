use crate::ext::ObjectIdExt;
use crate::Tag;

impl<'repo> Tag<'repo> {
    /// Decode this tag and return the id of its target.
    pub fn target_id(&self) -> Result<crate::Id<'repo>, git_object::decode::Error> {
        git_object::TagRefIter::from_bytes(&self.data)
            .target_id()
            .map(|id| id.attach(self.repo))
    }
}
