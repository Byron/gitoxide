use crate::ext::ReferenceExt;
use crate::types::RevSpecDetached;
use crate::{Id, Reference, Repository, RevSpec};

///
pub mod parse {
    use crate::bstr::BStr;
    use crate::types::RevSpecDetached;
    use crate::RevSpec;
    use crate::{object, Repository};
    use git_hash::ObjectId;
    use git_revision::spec::parse;
    use git_revision::spec::parse::delegate::{self, PeelTo, ReflogLookup, SiblingBranch, Traversal};
    use smallvec::SmallVec;
    use std::collections::HashSet;

    /// The error returned by [`crate::Repository::rev_parse()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(
            "The short hash {prefix} matched both the reference {} and the object(s) {}", reference.name, oids.iter().map(|oid| oid.to_string()).collect::<Vec<_>>().join(", ")
        )]
        AmbiguousRefAndObject {
            /// The prefix to look for.
            prefix: git_hash::Prefix,
            /// The reference matching the prefix.
            reference: git_ref::Reference,
            /// The object's ids that were matching the prefix as well.
            oids: HashSet<git_hash::ObjectId>,
        },
        #[error(transparent)]
        IdFromHex(#[from] git_hash::decode::Error),
        #[error(transparent)]
        FindReference(#[from] git_ref::file::find::existing::Error),
        #[error(transparent)]
        FindObject(#[from] object::find::existing::OdbError),
        #[error(transparent)]
        PeelToKind(#[from] object::peel::to_kind::Error),
        #[error("Object {oid} was a {actual}, but needed it to be a {expected}")]
        ObjectKind {
            oid: git_hash::ObjectId,
            actual: git_object::Kind,
            expected: git_object::Kind,
        },
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

    /// A hint to know which object kind to prefer if multiple objects match a prefix.
    ///
    /// This disambiguation mechanism is applied only if there is no disambiguation hints in the spec itself.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ObjectKindHint {
        /// Pick objects that are commits themselves.
        Commit,
        /// Pick objects that can be peeled into a commit, i.e. commits themselves or tags which are peeled until a commit is found.
        Committish,
        /// Pick objects that are trees themselves.
        Tree,
        /// Pick objects that can be peeled into a tree, i.e. trees themselves or tags which are peeled until a tree is found or commits
        /// whose tree is chosen.
        Treeish,
        /// Pick objects that are blobs.
        Blob,
    }

    impl Default for RefsHint {
        fn default() -> Self {
            RefsHint::PreferObjectOnFullLengthHexShaUseRefOtherwise
        }
    }

    /// Options for use in [`RevSpec::from_bstr()`].
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Options {
        /// What to do if both refs and object names match the same input.
        pub refs_hint: RefsHint,
        /// The hint to use when encountering multiple object matching a prefix.
        ///
        /// If `None`, the rev-spec itself must disambiguate the object by drilling down to desired kinds or applying
        /// other disambiguating transformations.
        pub object_kind_hint: Option<ObjectKindHint>,
    }

    impl<'repo> RevSpec<'repo> {
        /// Parse `spec` and use information from `repo` to resolve it, using `opts` to learn how to deal with ambiguity.
        pub fn from_bstr<'a>(spec: impl Into<&'a BStr>, repo: &'repo Repository, opts: Options) -> Result<Self, Error> {
            fn zero_or_one_objects_or_ambguity_err(
                candidates: Option<HashSet<ObjectId>>,
                prefix: Option<git_hash::Prefix>,
            ) -> Result<Option<ObjectId>, Error> {
                match candidates {
                    None => Ok(None),
                    Some(candidates) => {
                        match candidates.len() {
                            0 => unreachable!(
                                "BUG: let's avoid still being around if no candidate matched the requirements"
                            ),
                            1 => Ok(candidates.into_iter().next()),
                            _ => Err(Error::AmbiguousPrefix {
                                // TODO: resolve object types and provide additional information about them
                                prefix: prefix.expect("set when obtaining candidates"),
                            }),
                        }
                    }
                }
            }
            let mut delegate = Delegate {
                refs: Default::default(),
                objs: Default::default(),
                idx: 0,
                kind: None,
                err: Vec::new(),
                prefix: Default::default(),
                last_call_was_disambiguate_prefix: Default::default(),
                opts,
                repo,
            };
            let spec = match git_revision::spec::parse(spec.into(), &mut delegate) {
                Err(git_revision::spec::parse::Error::Delegate) => {
                    assert!(
                        !delegate.err.is_empty(),
                        "BUG: must have recorded at least one err if a delegate error was reported"
                    );
                    if delegate.err.len() == 1 {
                        return Err(delegate.err.remove(0));
                    }
                    // TODO: is there a way to not degenerate the error but rather build some sort of error chain?
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
                        from: zero_or_one_objects_or_ambguity_err(delegate.objs[0].take(), delegate.prefix[0])?,
                        to_ref: delegate.refs[1].take(),
                        to: zero_or_one_objects_or_ambguity_err(delegate.objs[1].take(), delegate.prefix[1])?,
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
        objs: [Option<HashSet<git_hash::ObjectId>>; 2],
        idx: usize,
        kind: Option<git_revision::spec::Kind>,

        opts: Options,
        err: Vec<Error>,
        /// The ambiguous prefix obtained during a call to `disambiguate_prefix()`.
        prefix: [Option<git_hash::Prefix>; 2],
        /// If true, we didn't try to do any other transformation which might have helped with disambiguation.
        last_call_was_disambiguate_prefix: [bool; 2],

        repo: &'repo Repository,
    }

    impl<'repo> parse::Delegate for Delegate<'repo> {
        fn done(&mut self) {
            self.follow_refs_to_objects_if_needed();
            self.disambiguate_objects_by_fallback_hint();
        }
    }

    impl<'repo> Delegate<'repo> {
        fn disambiguate_objects_by_fallback_hint(&mut self) {
            if self.last_call_was_disambiguate_prefix[self.idx] {
                self.unset_disambiguate_call();

                if let Some(objs) = self.objs[self.idx].as_mut() {
                    let repo = self.repo;
                    let errors: Vec<_> = match self.opts.object_kind_hint {
                        Some(kind_hint) => match kind_hint {
                            ObjectKindHint::Treeish | ObjectKindHint::Committish => {
                                let kind = match kind_hint {
                                    ObjectKindHint::Treeish => git_object::Kind::Tree,
                                    ObjectKindHint::Committish => git_object::Kind::Commit,
                                    _ => unreachable!("BUG: we narrow possibilities above"),
                                };
                                objs.iter()
                                    .filter_map(|obj| peel(repo, obj, kind).err().map(|err| (*obj, err)))
                                    .collect()
                            }
                            ObjectKindHint::Tree | ObjectKindHint::Commit | ObjectKindHint::Blob => {
                                let kind = match kind_hint {
                                    ObjectKindHint::Tree => git_object::Kind::Tree,
                                    ObjectKindHint::Commit => git_object::Kind::Commit,
                                    ObjectKindHint::Blob => git_object::Kind::Blob,
                                    _ => unreachable!("BUG: we narrow possibilities above"),
                                };
                                objs.iter()
                                    .filter_map(|obj| require_object_kind(repo, obj, kind).err().map(|err| (*obj, err)))
                                    .collect()
                            }
                        },
                        None => return,
                    };

                    if errors.len() == objs.len() {
                        self.err.extend(errors.into_iter().map(|(_, err)| err));
                    } else {
                        for (obj, err) in errors {
                            objs.remove(&obj);
                            self.err.push(err);
                        }
                    }
                }
            }
        }
        fn follow_refs_to_objects_if_needed(&mut self) -> Option<()> {
            assert_eq!(self.refs.len(), self.objs.len());
            for (r, obj) in self.refs.iter().zip(self.objs.iter_mut()) {
                if let (_ref_opt @ Some(ref_), obj_opt @ None) = (r, obj) {
                    match ref_.target.try_id() {
                        Some(id) => obj_opt.get_or_insert_with(HashSet::default).insert(id.into()),
                        None => todo!("follow ref to get direct target object"),
                    };
                };
            }
            Some(())
        }

        fn unset_disambiguate_call(&mut self) {
            self.last_call_was_disambiguate_prefix[self.idx] = false;
        }
    }

    impl<'repo> delegate::Revision for Delegate<'repo> {
        fn find_ref(&mut self, name: &BStr) -> Option<()> {
            self.unset_disambiguate_call();
            if !self.err.is_empty() && self.refs[self.idx].is_some() {
                return None;
            }
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
            self.last_call_was_disambiguate_prefix[self.idx] = true;
            let mut candidates = Some(HashSet::default());
            self.prefix[self.idx] = Some(prefix);
            match self
                .repo
                .objects
                .lookup_prefix(prefix, candidates.as_mut())
                .map_err(object::find::existing::OdbError::Find)
            {
                Err(err) => {
                    self.err.push(err.into());
                    None
                }
                Ok(None) => {
                    self.err.push(Error::PrefixNotFound { prefix });
                    None
                }
                Ok(Some(Ok(_) | Err(()))) => {
                    assert!(self.objs[self.idx].is_none(), "BUG: cannot set the same prefix twice");
                    let candidates = candidates.expect("set above");
                    match self.opts.refs_hint {
                        RefsHint::PreferObjectOnFullLengthHexShaUseRefOtherwise
                            if prefix.hex_len()
                                == candidates.iter().next().expect("at least one").kind().len_in_hex() =>
                        {
                            self.objs[self.idx] = Some(candidates);
                            Some(())
                        }
                        RefsHint::PreferObject => {
                            self.objs[self.idx] = Some(candidates);
                            Some(())
                        }
                        RefsHint::PreferRef
                        | RefsHint::PreferObjectOnFullLengthHexShaUseRefOtherwise
                        | RefsHint::Fail => match self.repo.refs.find(&prefix.to_string()) {
                            Ok(ref_) => {
                                assert!(self.refs[self.idx].is_none(), "BUG: cannot set the same ref twice");
                                if self.opts.refs_hint == RefsHint::Fail {
                                    self.refs[self.idx] = Some(ref_.clone());
                                    self.err.push(Error::AmbiguousRefAndObject {
                                        prefix,
                                        reference: ref_,
                                        oids: candidates,
                                    });
                                    None
                                } else {
                                    self.refs[self.idx] = Some(ref_);
                                    Some(())
                                }
                            }
                            Err(_) => {
                                self.objs[self.idx] = Some(candidates);
                                Some(())
                            }
                        },
                    }
                }
            }
        }

        fn reflog(&mut self, _query: ReflogLookup) -> Option<()> {
            self.unset_disambiguate_call();
            todo!()
        }

        fn nth_checked_out_branch(&mut self, _branch_no: usize) -> Option<()> {
            self.unset_disambiguate_call();
            todo!()
        }

        fn sibling_branch(&mut self, _kind: SiblingBranch) -> Option<()> {
            self.unset_disambiguate_call();
            todo!()
        }
    }

    impl<'repo> delegate::Navigate for Delegate<'repo> {
        fn traverse(&mut self, _kind: Traversal) -> Option<()> {
            self.unset_disambiguate_call();
            todo!()
        }

        fn peel_until(&mut self, kind: PeelTo<'_>) -> Option<()> {
            self.unset_disambiguate_call();
            self.follow_refs_to_objects_if_needed()?;

            let mut replacements = SmallVec::<[(git_hash::ObjectId, git_hash::ObjectId); 1]>::default();
            let mut errors = Vec::new();
            let objs = self.objs[self.idx].as_mut()?;

            match kind {
                PeelTo::ValidObject => {
                    for obj in objs.iter() {
                        match self.repo.find_object(*obj) {
                            Ok(_) => {}
                            Err(err) => {
                                errors.push((*obj, err.into()));
                            }
                        };
                    }
                }
                PeelTo::ObjectKind(kind) => {
                    let repo = self.repo;
                    let peel = |obj| peel(repo, obj, kind);
                    for obj in objs.iter() {
                        match peel(obj) {
                            Ok(replace) => replacements.push((*obj, replace)),
                            Err(err) => errors.push((*obj, err)),
                        }
                    }
                }
                PeelTo::Path(_path) => todo!("lookup path"),
                PeelTo::RecursiveTagObject => todo!("recursive tag object"),
            }

            if errors.len() == objs.len() {
                self.err.extend(errors.into_iter().map(|(_, err)| err));
                None
            } else {
                for (obj, err) in errors {
                    objs.remove(&obj);
                    self.err.push(err);
                }
                for (find, replace) in replacements {
                    objs.remove(&find);
                    objs.insert(replace);
                }
                Some(())
            }
        }

        fn find(&mut self, _regex: &BStr, _negated: bool) -> Option<()> {
            self.unset_disambiguate_call();
            todo!()
        }

        fn index_lookup(&mut self, _path: &BStr, _stage: u8) -> Option<()> {
            self.unset_disambiguate_call();
            todo!()
        }
    }

    impl<'repo> delegate::Kind for Delegate<'repo> {
        fn kind(&mut self, _kind: git_revision::spec::Kind) -> Option<()> {
            todo!("kind, deal with ^ and .. and ... correctly")
        }
    }

    fn peel(repo: &Repository, obj: &git_hash::oid, kind: git_object::Kind) -> Result<git_hash::ObjectId, Error> {
        let mut obj = repo.find_object(obj)?;
        obj = obj.peel_to_kind(kind)?;
        debug_assert_eq!(obj.kind, kind, "bug in Object::peel_to_kind() which didn't deliver");
        Ok(obj.id)
    }

    fn require_object_kind(repo: &Repository, obj: &git_hash::oid, kind: git_object::Kind) -> Result<(), Error> {
        let obj = repo.find_object(obj)?;
        if obj.kind == kind {
            Ok(())
        } else {
            Err(Error::ObjectKind {
                actual: obj.kind,
                expected: kind,
                oid: obj.id,
            })
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
