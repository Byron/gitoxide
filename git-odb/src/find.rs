/// A way to indicate if a lookup, despite successful, was ambiguous or yielded exactly
/// one result in the particular index.
// TODO: find better name, ambiguous with git_pack::index::PrefixLookupResult (entry_index inside)
pub type PrefixLookupResult = Result<git_hash::ObjectId, ()>;

/// A potentially ambiguous prefix for use with `Handle::disambiguate_prefix()`.
#[derive(Debug, Copy, Clone)]
pub struct PotentialPrefix {
    id: git_hash::ObjectId,
    hex_len: usize,
}

impl PotentialPrefix {
    /// Create a new potentially ambiguous prefix from an `id` and the desired minimal `hex_len`.
    ///
    /// It is considered ambiguous until it's disambiguated by validating that there is only a single object
    /// matching this prefix.
    pub fn new(id: impl Into<git_hash::ObjectId>, hex_len: usize) -> Result<Self, git_hash::prefix::Error> {
        let id = id.into();
        git_hash::Prefix::new(&id, hex_len)?;
        Ok(PotentialPrefix { id, hex_len })
    }

    /// Transform ourselves into a `Prefix` with our current hex lengths.
    pub fn to_prefix(&self) -> git_hash::Prefix {
        git_hash::Prefix::new(self.id, self.hex_len).expect("our hex-len to always be in bounds")
    }

    pub(crate) fn inc_hex_len(&mut self) {
        self.hex_len += 1;
        assert!(self.hex_len <= self.id.kind().len_in_hex());
    }

    pub(crate) fn id(&self) -> &git_hash::oid {
        &self.id
    }

    pub(crate) fn hex_len(&self) -> usize {
        self.hex_len
    }
}

///
pub mod existing {
    use git_hash::ObjectId;

    /// The error returned by the [`find(…)`][crate::FindExt::find()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<T: std::error::Error + 'static> {
        #[error(transparent)]
        Find(T),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
    }
}

///
pub mod existing_object {
    use git_hash::ObjectId;

    /// The error returned by the various [`find_*()`][crate::FindExt::find_commit()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<T: std::error::Error + 'static> {
        #[error(transparent)]
        Find(T),
        #[error(transparent)]
        Decode(git_object::decode::Error),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
        #[error("Expected object of kind {} something else", .expected)]
        ObjectKind { expected: git_object::Kind },
    }
}

///
pub mod existing_iter {
    use git_hash::ObjectId;

    /// The error returned by the various [`find_*_iter()`][crate::FindExt::find_commit_iter()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<T: std::error::Error + 'static> {
        #[error(transparent)]
        Find(T),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
        #[error("Expected object of kind {} something else", .expected)]
        ObjectKind { expected: git_object::Kind },
    }
}
