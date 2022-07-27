use crate::ext::ReferenceExt;
use crate::types::RevSpecDetached;
use crate::{Id, Reference, Repository, RevSpec};

///
pub mod parse;

mod impls {
    use crate::RevSpec;

    impl<'repo> PartialEq for RevSpec<'repo> {
        fn eq(&self, other: &Self) -> bool {
            self.inner.kind == other.inner.kind
                && self.inner.from == other.inner.from
                && self.inner.to == other.inner.to
        }
    }

    impl<'repo> Eq for RevSpec<'repo> {}
}

/// Initialization
impl<'repo> RevSpec<'repo> {
    /// Create a single specification which points to `id`.
    pub fn from_id(id: Id<'repo>) -> Self {
        RevSpec {
            inner: RevSpecDetached {
                from_ref: None,
                from: Some(id.inner),
                to: None,
                to_ref: None,
                kind: None,
            },
            repo: id.repo,
        }
    }
}

/// Access
impl<'repo> RevSpec<'repo> {
    /// Detach the `Repository` from this instance, leaving only plain data that can be moved freely and serialized.
    pub fn detach(self) -> RevSpecDetached {
        self.inner
    }

    /// Some revision specifications leave information about reference names which are returned as `(from-ref, to-ref)` here, e.g.
    /// `HEAD@{-1}..main` might be (`refs/heads/previous-branch`, `refs/heads/main`).
    ///
    /// Note that no reference name is known when revisions are specified by prefix as with `v1.2.3-10-gabcd1234`.
    // TODO: tests
    pub fn into_names(self) -> (Option<Reference<'repo>>, Option<Reference<'repo>>) {
        // TODO: assure we can set the reference also when it is only implied, like with `@{1}`.
        let repo = self.repo;
        let this = self.inner;
        (
            this.from_ref.map(|r| r.attach(repo)),
            this.to_ref.map(|r| r.attach(repo)),
        )
    }

    /// The object from which to start a range, or the only revision as specified by e.g. `@` or `HEAD`.
    ///
    /// Note that this can be `None` for ranges like e.g. `^HEAD`, `..@`, `...v1.0` or similar.
    pub fn from(&self) -> Option<Id<'repo>> {
        self.inner.from.map(|id| Id::from_id(id, self.repo))
    }
    /// The object at which the range ends, as in e.g. `...HEAD` or `...feature`.
    ///
    /// Note that this can be `None` in case of single revisions like `HEAD@{1}` or `HEAD...`.
    pub fn to(&self) -> Option<Id<'repo>> {
        self.inner.to.map(|id| Id::from_id(id, self.repo))
    }

    /// Return the single object represented by this instance, or `None` if it is a range of any kind.
    pub fn single(&self) -> Option<Id<'repo>> {
        self.inner
            .from
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

    /// Returns the kind of this rev-spec.
    pub fn kind(&self) -> git_revision::spec::Kind {
        self.inner.kind.unwrap_or(git_revision::spec::Kind::Single)
    }
}

/// Access
impl RevSpecDetached {
    /// Attach `repo` to ourselves for more convenient types.
    pub fn attach(self, repo: &Repository) -> RevSpec<'_> {
        RevSpec { inner: self, repo }
    }
    /// Some revision specifications leave information about reference names which are returned as `(from-ref, to-ref)` here, e.g.
    /// `HEAD@{-1}..main` might be (`refs/heads/previous-branch`, `refs/heads/main`).
    ///
    /// Note that no reference name is known when revisions are specified by prefix as with `v1.2.3-10-gabcd1234`.
    // TODO: tests
    pub fn into_names(self) -> (Option<git_ref::Reference>, Option<git_ref::Reference>) {
        // TODO: assure we can set the reference also when it is only implied, like with `@{1}`.
        (self.from_ref, self.to_ref)
    }

    /// The object from which to start a range, or the only revision as specified by e.g. `@` or `HEAD`.
    ///
    /// Note that this can be `None` for ranges like e.g. `^HEAD`, `..@`, `...v1.0` or similar.
    pub fn from(&self) -> Option<git_hash::ObjectId> {
        self.from
    }
    /// The object at which the range ends, as in e.g. `...HEAD` or `...feature`.
    ///
    /// Note that this can be `None` in case of single revisions like `HEAD@{1}` or `HEAD...`.
    pub fn to(&self) -> Option<git_hash::ObjectId> {
        self.to
    }

    /// Return the single object represented by this instance, or `None` if it is a range of any kind.
    pub fn single(&self) -> Option<git_hash::ObjectId> {
        self.from
            .and_then(|id| matches!(self.kind(), git_revision::spec::Kind::Single).then(|| id))
    }

    /// Return `(kind being merge-base or range, from-id, to-id)` if our `kind` is not describing a single revision.
    ///
    /// Note that `...HEAD` is equivalent to `HEAD...HEAD` and `HEAD..` is equivalent to `HEAD..HEAD`. If this is not desirable,
    /// access [`from()`][RevSpec::from()] and [`to()`][RevSpec::to()] individually after validating that [`kind()`][RevSpec::kind()]
    /// is indeed not a single revision.
    // TODO: test
    pub fn range(&self) -> Option<(git_revision::spec::Kind, git_hash::ObjectId, git_hash::ObjectId)> {
        (!matches!(self.kind(), git_revision::spec::Kind::Single)).then(|| {
            (
                self.kind(),
                self.from().or_else(|| self.to()).expect("at least one id is set"),
                self.to().or_else(|| self.from()).expect("at least one id is set"),
            )
        })
    }

    /// Returns the kind of this detached rev-spec.
    pub fn kind(&self) -> git_revision::spec::Kind {
        self.kind.unwrap_or(git_revision::spec::Kind::Single)
    }
}
