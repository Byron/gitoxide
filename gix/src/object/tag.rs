use crate::{ext::ObjectIdExt, Tag};

impl<'repo> Tag<'repo> {
    /// Decode the entire tag object and return it for accessing all tag information.
    ///
    /// This never allocates.
    ///
    /// Note that the returned commit object does make lookup easy and should be
    /// used for successive calls to string-ish information to avoid decoding the object
    /// more than once.
    pub fn decode(&self) -> Result<gix_object::TagRef<'_>, gix_object::decode::Error> {
        gix_object::TagRef::from_bytes(&self.data)
    }

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
