#![allow(missing_docs)]
use crate::{Id, RevSpec};

///
pub mod parse {

    use crate::Repository;
    use crate::RevSpec;

    /// The error returned by [`crate::Repository::rev_parse()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        IdFromHex(#[from] git_hash::decode::Error),
        #[error(transparent)]
        Find(#[from] crate::object::find::existing::OdbError),
    }

    impl<'repo> RevSpec<'repo> {
        pub fn from_bytes(_spec: impl AsRef<[u8]>, _repo: &Repository) -> Result<Self, Error> {
            todo!()
        }
    }
}

/// Access
impl<'repo> RevSpec<'repo> {
    /// The object from which to start a range, or the only revision as specified by e.g. `@` or `HEAD`.
    ///
    /// Note that this can be `None` for ranges like e.g. `^HEAD`, `..@`, `...v1.0` or similar.
    pub fn from(&self) -> Option<Id<'repo>> {
        self.from.map(|id| Id::from_id(id, self.repo))
    }
    /// The object at which the range ends, as in e.g. `...HEAD` or `...feature`.
    ///
    /// Note that this can be `None` in case of single revisions like `HEAD@{1}` or `HEAD...`.
    pub fn to(&self) -> Option<Id<'repo>> {
        self.to.map(|id| Id::from_id(id, self.repo))
    }

    /// Return the single object represented by this instance, or `None` if it is a range of any kind.
    pub fn single(&self) -> Option<Id<'repo>> {
        self.from
            .and_then(|id| matches!(self.kind(), git_revision::spec::Kind::Single).then(|| Id::from_id(id, self.repo)))
    }

    /// Return `(kind being merge-base or range, from-id, to-id)` if our `kind` is not describing a single revision.
    ///
    /// Note that `...HEAD` is equivalent to `HEAD...HEAD` and `HEAD..` is equivalent to `HEAD..HEAD`. If this is not desirable,
    /// access [`from()`][RevSpec::from()] and [`to()`][RevSpec::to()] individually after validating that [`kind()`][RevSpec::kind()]
    /// is indeed not a single revision.
    // TODO: test
    pub fn range(&self) -> Option<(git_revision::spec::Kind, Id<'repo>, Id<'repo>)> {
        (!matches!(self.kind(), git_revision::spec::Kind::Single)).then(|| {
            (
                self.kind(),
                self.from().or_else(|| self.to()).expect("at least one id is set"),
                self.to().or_else(|| self.from()).expect("at least one id is set"),
            )
        })
    }

    pub fn kind(&self) -> git_revision::spec::Kind {
        self.kind.unwrap_or(git_revision::spec::Kind::Single)
    }
}
