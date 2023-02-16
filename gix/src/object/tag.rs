use crate::{ext::ObjectIdExt, Tag};

impl<'repo> Tag<'repo> {
    /// Decode this tag partially and return the id of its target.
    pub fn target_id(&self) -> Result<crate::Id<'repo>, gix_object::decode::Error> {
        gix_object::TagRefIter::from_bytes(&self.data)
            .target_id()
            .map(|id| id.attach(self.repo))
    }

    /// Decode this tag partially and return the tagger, if the field exists.
    pub fn tagger(&self) -> Result<Option<gix_actor::SignatureRef<'_>>, gix_object::decode::Error> {
        gix_object::TagRefIter::from_bytes(&self.data).tagger()
    }
}
