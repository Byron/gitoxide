use crate::Store;

impl Store {
    /// The root path at which we expect to find all objects and packs.
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// The kind of object hash to assume when dealing with pack indices and pack data files.
    pub fn object_hash(&self) -> git_hash::Kind {
        self.object_hash
    }

    /// Whether or not we are allowed to use multi-pack indices
    pub fn use_multi_pack_index(&self) -> bool {
        self.use_multi_pack_index
    }

    /// An iterator over replacements from object-ids `X` to `X-replaced` as `(X, X-replaced)`, sorted by the original id `X`.
    pub fn replacements(&self) -> impl Iterator<Item = (git_hash::ObjectId, git_hash::ObjectId)> + '_ {
        self.replacements.iter().cloned()
    }
}
