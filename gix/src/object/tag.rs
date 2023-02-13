use crate::{ext::ObjectIdExt, Tag};

impl<'repo> Tag<'repo> {
    /// Decode this tag partially and return the id of its target.
    pub fn target_id(&self) -> Result<crate::Id<'repo>, git_object::decode::Error> {
        git_object::TagRefIter::from_bytes(&self.data)
            .target_id()
            .map(|id| id.attach(self.repo))
    }

    /// Decode this tag partially and return the tagger, if the field exists.
    pub fn tagger(&self) -> Result<Option<git_actor::SignatureRef<'_>>, git_object::decode::Error> {
        git_object::TagRefIter::from_bytes(&self.data).tagger()
    }
}
