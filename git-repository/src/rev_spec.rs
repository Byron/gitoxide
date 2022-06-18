#![allow(missing_docs)]
use crate::ext::ReferenceExt;
use crate::types::RevSpecDetached;
use crate::{Id, Reference, Repository, RevSpec};

///
pub mod parse {
    use crate::bstr::{BStr, ByteSlice};
    use crate::types::RevSpecDetached;
    use crate::Repository;
    use crate::RevSpec;
    use git_revision::spec::parse;
    use git_revision::spec::parse::delegate::{self, PeelTo, ReflogLookup, SiblingBranch, Traversal};

    /// The error returned by [`crate::Repository::rev_parse()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        IdFromHex(#[from] git_hash::decode::Error),
        #[error(transparent)]
        FindReference(#[from] git_ref::file::find::existing::Error),
        #[error(transparent)]
        FindObject(#[from] crate::object::find::existing::OdbError),
        #[error(transparent)]
        Parse(#[from] git_revision::spec::parse::Error),
        #[error("An object prefixed {} could not be found", .prefix)]
        PrefixNotFound { prefix: git_hash::Prefix },
        #[error("Found more than one object prefixed with {}", .prefix)]
        AmbiguousPrefix { prefix: git_hash::Prefix },
        #[error("{}", .combined_message)]
        Multi { combined_message: String },
    }

    /// A hint to know what to do if refs and object names are equal.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RefsHint {
        /// This is the default, and leads to specs that look like objects identified by full hex sha and are objets to be used
        /// instead of similarly named references. The latter is not typical but can absolutely happen by accident.
        /// If the object prefix is shorter than the maximum hash length of the repository, use the reference instead, which is
        /// preferred as there are many valid object names like `beef` and `cafe` that are short and both valid and typical prefixes
        /// for objects.
        /// Git chooses this as default as well, even though it means that every object prefix is also looked up as ref.
        PreferObjectOnFullLengthHexShaUseRefOtherwise,
        /// No matter what, if it looks like an object prefix and has an object, use it.
        /// Note that no ref-lookup is made here which is the fastest option.
        PreferObject,
        /// When an object is found for a given prefix, also check if a reference exists with that name and if it does,
        /// use that moving forward.
        PreferRef,
        /// If there is an ambiguous situation, instead of silently choosing one over the other, fail instead.
        Fail,
    }

    impl Default for RefsHint {
        fn default() -> Self {
            RefsHint::PreferObject
        }
    }

    #[derive(Debug, Default, Copy, Clone)]
    pub struct Options {
        pub refs_hint: RefsHint,
    }

    impl<'repo> RevSpec<'repo> {
        pub fn from_bstr(
            spec: impl AsRef<BStr>,
            repo: &'repo Repository,
            Options { refs_hint: _ }: Options,
        ) -> Result<Self, Error> {
            let mut delegate = Delegate {
                refs: Default::default(),
                objs: Default::default(),
                idx: 0,
                kind: None,
                err: Vec::new(),
                repo,
            };
            let spec = match git_revision::spec::parse(spec.as_ref().as_bstr(), &mut delegate) {
                Err(git_revision::spec::parse::Error::Delegate) => {
                    assert!(
                        !delegate.err.is_empty(),
                        "BUG: must have recorded at least one err if a delegate error was reported"
                    );
                    if delegate.err.len() == 1 {
                        return Err(delegate.err.remove(0));
                    }
                    // TODO: is there a way to not degenerate the error easily?
                    return Err(Error::Multi {
                        combined_message: delegate
                            .err
                            .iter()
                            .map(|err| err.to_string())
                            .collect::<Vec<_>>()
                            .join("\n"),
                    });
                }
                Err(err) => return Err(err.into()),
                Ok(()) => RevSpec {
                    inner: RevSpecDetached {
                        from_ref: delegate.refs[0].take(),
                        from: delegate.objs[0],
                        to_ref: delegate.refs[1].take(),
                        to: delegate.objs[1],
                        kind: delegate.kind,
                    },
                    repo,
                },
            };
            Ok(spec)
        }
    }

    #[allow(dead_code)]
    struct Delegate<'repo> {
        refs: [Option<git_ref::Reference>; 2],
        objs: [Option<git_hash::ObjectId>; 2],

        idx: usize,

        kind: Option<git_revision::spec::Kind>,
        repo: &'repo Repository,
        err: Vec<Error>,
    }

    impl<'repo> parse::Delegate for Delegate<'repo> {
        fn done(&mut self) {
            self.follow_refs_to_objects_if_needed();
            assert!(
                self.err.is_empty(),
                "BUG: cannot have errors and still arrive here - delegate must return None after registering an error"
            )
        }
    }

    impl<'repo> Delegate<'repo> {
        fn follow_refs_to_objects_if_needed(&mut self) -> Option<()> {
            assert_eq!(self.refs.len(), self.objs.len());
            for (r, obj) in self.refs.iter().zip(self.objs.iter_mut()) {
                if let (_ref_opt @ Some(ref_), obj_opt @ None) = (r, obj) {
                    match ref_.target.try_id() {
                        Some(id) => *obj_opt = Some(id.into()),
                        None => todo!("follow ref to get direct target object"),
                    }
                }
            }
            Some(())
        }
    }

    impl<'repo> delegate::Revision for Delegate<'repo> {
        fn find_ref(&mut self, name: &BStr) -> Option<()> {
            match self.repo.refs.find(name) {
                Ok(r) => {
                    assert!(self.refs[self.idx].is_none(), "BUG: cannot set the same ref twice");
                    self.refs[self.idx] = Some(r);
                    Some(())
                }
                Err(err) => {
                    self.err.push(err.into());
                    None
                }
            }
        }

        fn disambiguate_prefix(
            &mut self,
            prefix: git_hash::Prefix,
            _must_be_commit: Option<delegate::PrefixHint<'_>>,
        ) -> Option<()> {
            match self
                .repo
                .objects
                .lookup_prefix(prefix)
                .map_err(crate::object::find::existing::OdbError::Find)
            {
                Err(err) => {
                    self.err.push(err.into());
                    None
                }
                Ok(None) => {
                    self.err.push(Error::PrefixNotFound { prefix });
                    None
                }
                Ok(Some(Err(()))) => {
                    self.err.push(Error::AmbiguousPrefix { prefix });
                    None
                }
                Ok(Some(Ok(id))) => {
                    assert!(self.objs[self.idx].is_none(), "BUG: cannot set the same prefix twice");
                    self.objs[self.idx] = Some(id);
                    Some(())
                }
            }
        }

        fn reflog(&mut self, _query: ReflogLookup) -> Option<()> {
            todo!()
        }

        fn nth_checked_out_branch(&mut self, _branch_no: usize) -> Option<()> {
            todo!()
        }

        fn sibling_branch(&mut self, _kind: SiblingBranch) -> Option<()> {
            todo!()
        }
    }

    impl<'repo> delegate::Navigate for Delegate<'repo> {
        fn traverse(&mut self, _kind: Traversal) -> Option<()> {
            todo!()
        }

        fn peel_until(&mut self, kind: PeelTo<'_>) -> Option<()> {
            self.follow_refs_to_objects_if_needed()?;

            match kind {
                PeelTo::ValidObject => {
                    if let Err(err) = self.repo.find_object(self.objs[self.idx]?) {
                        self.err.push(err.into());
                        return None;
                    }
                }
                PeelTo::ObjectKind(_kind) => todo!("peel to kind"),
                PeelTo::Path(_path) => todo!("lookup path"),
                PeelTo::RecursiveTagObject => todo!("recursive tag object"),
            }
            Some(())
        }

        fn find(&mut self, _regex: &BStr, _negated: bool) -> Option<()> {
            todo!()
        }

        fn index_lookup(&mut self, _path: &BStr, _stage: u8) -> Option<()> {
            todo!()
        }
    }

    impl<'repo> delegate::Kind for Delegate<'repo> {
        fn kind(&mut self, _kind: git_revision::spec::Kind) -> Option<()> {
            todo!("kind, deal with ^ and .. and ... correctly")
        }
    }
}

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

    pub fn kind(&self) -> git_revision::spec::Kind {
        self.kind.unwrap_or(git_revision::spec::Kind::Single)
    }
}
