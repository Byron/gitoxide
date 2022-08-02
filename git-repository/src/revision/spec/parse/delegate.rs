use super::{Delegate, Error, ObjectKindHint, RefsHint};
use crate::bstr::{BStr, ByteSlice};
use crate::ext::{ObjectIdExt, ReferenceExt};
use crate::{object, Repository};
use git_hash::ObjectId;
use git_revision::spec::parse;
use git_revision::spec::parse::delegate::{self, PeelTo, ReflogLookup, SiblingBranch, Traversal};
use git_traverse::commit::Sorting;
use smallvec::SmallVec;
use std::collections::HashSet;

impl<'repo> Delegate<'repo> {
    pub fn new(repo: &'repo Repository, opts: crate::revision::spec::parse::Options) -> Self {
        Delegate {
            refs: Default::default(),
            objs: Default::default(),
            ambiguous_objects: Default::default(),
            idx: 0,
            kind: None,
            err: Vec::new(),
            prefix: Default::default(),
            last_call_was_disambiguate_prefix: Default::default(),
            opts,
            repo,
        }
    }

    pub fn into_err(mut self) -> Error {
        let repo = self.repo;
        for err in self
            .ambiguous_objects
            .iter_mut()
            .zip(self.prefix)
            .filter_map(|(a, b)| a.take().filter(|candidates| candidates.len() > 1).zip(b))
            .map(|(candidates, prefix)| Error::ambiguous(candidates, prefix, repo))
            .rev()
        {
            self.err.insert(0, err);
        }
        Error::from_errors(self.err)
    }

    pub fn into_rev_spec(mut self) -> Result<crate::revision::Spec<'repo>, Error> {
        fn zero_or_one_objects_or_ambguity_err(
            mut candidates: [Option<HashSet<ObjectId>>; 2],
            prefix: [Option<git_hash::Prefix>; 2],
            mut errors: Vec<Error>,
            repo: &Repository,
        ) -> Result<[Option<ObjectId>; 2], Error> {
            let mut out = [None, None];
            for ((candidates, prefix), out) in candidates.iter_mut().zip(prefix).zip(out.iter_mut()) {
                let candidates = candidates.take();
                match candidates {
                    None => *out = None,
                    Some(candidates) => {
                        match candidates.len() {
                            0 => unreachable!(
                                "BUG: let's avoid still being around if no candidate matched the requirements"
                            ),
                            1 => {
                                *out = candidates.into_iter().next();
                            }
                            _ => {
                                errors.insert(
                                    0,
                                    Error::ambiguous(candidates, prefix.expect("set when obtaining candidates"), repo),
                                );
                                return Err(Error::from_errors(errors));
                            }
                        };
                    }
                };
            }
            Ok(out)
        }

        fn kind_to_spec(
            kind: Option<git_revision::spec::Kind>,
            [first, second]: [Option<ObjectId>; 2],
        ) -> git_revision::Spec {
            use git_revision::spec::Kind::*;
            match kind.unwrap_or_default() {
                IncludeReachable => git_revision::Spec::Include(first.expect("set by parser")),
                ExcludeReachable => git_revision::Spec::Exclude(first.expect("set by parser")),
                RangeBetween => git_revision::Spec::Range {
                    from: first.expect("set by parser"),
                    to: second.expect("set by parser"),
                },
                ReachableToMergeBase => git_revision::Spec::Merge {
                    theirs: first.expect("set by parser"),
                    ours: second.expect("set by parser"),
                },
                IncludeReachableFromParents => git_revision::Spec::IncludeOnlyParents(first.expect("set by parser")),
                ExcludeReachableFromParents => git_revision::Spec::ExcludeParents(first.expect("set by parser")),
            }
        }

        let range = zero_or_one_objects_or_ambguity_err(self.objs, self.prefix, self.err, self.repo)?;
        Ok(crate::revision::Spec {
            first_ref: self.refs[0].take(),
            second_ref: self.refs[1].take(),
            inner: kind_to_spec(self.kind, range),
            repo: self.repo,
        })
    }
}

impl<'repo> parse::Delegate for Delegate<'repo> {
    fn done(&mut self) {
        self.follow_refs_to_objects_if_needed();
        self.disambiguate_objects_by_fallback_hint(
            self.kind_implies_committish()
                .then(|| ObjectKindHint::Committish)
                .or(self.opts.object_kind_hint),
        );
    }
}

impl<'repo> Delegate<'repo> {
    fn kind_implies_committish(&self) -> bool {
        self.kind.unwrap_or(git_revision::spec::Kind::IncludeReachable) != git_revision::spec::Kind::IncludeReachable
    }
    fn disambiguate_objects_by_fallback_hint(&mut self, hint: Option<ObjectKindHint>) {
        if self.last_call_was_disambiguate_prefix[self.idx] {
            self.unset_disambiguate_call();

            if let Some(objs) = self.objs[self.idx].as_mut() {
                let repo = self.repo;
                let errors: Vec<_> = match hint {
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
        let repo = self.repo;
        for (r, obj) in self.refs.iter().zip(self.objs.iter_mut()) {
            if let (_ref_opt @ Some(ref_), obj_opt @ None) = (r, obj) {
                if let Some(id) = ref_.target.try_id().map(ToOwned::to_owned).or_else(|| {
                    ref_.clone()
                        .attach(repo)
                        .peel_to_id_in_place()
                        .ok()
                        .map(|id| id.detach())
                }) {
                    obj_opt.get_or_insert_with(HashSet::default).insert(id);
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
        match self.repo.objects.lookup_prefix(prefix, candidates.as_mut()) {
            Err(err) => {
                self.err.push(object::find::existing::OdbError::Find(err).into());
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
                        if prefix.hex_len() == candidates.iter().next().expect("at least one").kind().len_in_hex() =>
                    {
                        self.ambiguous_objects[self.idx] = Some(candidates.clone());
                        self.objs[self.idx] = Some(candidates);
                        Some(())
                    }
                    RefsHint::PreferObject => {
                        self.ambiguous_objects[self.idx] = Some(candidates.clone());
                        self.objs[self.idx] = Some(candidates);
                        Some(())
                    }
                    RefsHint::PreferRef | RefsHint::PreferObjectOnFullLengthHexShaUseRefOtherwise | RefsHint::Fail => {
                        match self.repo.refs.find(&prefix.to_string()) {
                            Ok(ref_) => {
                                assert!(self.refs[self.idx].is_none(), "BUG: cannot set the same ref twice");
                                if self.opts.refs_hint == RefsHint::Fail {
                                    self.refs[self.idx] = Some(ref_.clone());
                                    self.err.push(Error::AmbiguousRefAndObject {
                                        prefix,
                                        reference: ref_,
                                    });
                                    self.err.push(Error::ambiguous(candidates, prefix, self.repo));
                                    None
                                } else {
                                    self.refs[self.idx] = Some(ref_);
                                    Some(())
                                }
                            }
                            Err(_) => {
                                self.ambiguous_objects[self.idx] = Some(candidates.clone());
                                self.objs[self.idx] = Some(candidates);
                                Some(())
                            }
                        }
                    }
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
    fn traverse(&mut self, kind: Traversal) -> Option<()> {
        self.unset_disambiguate_call();
        self.follow_refs_to_objects_if_needed()?;

        let mut replacements = SmallVec::<[(ObjectId, ObjectId); 1]>::default();
        let mut errors = Vec::new();
        let objs = self.objs[self.idx].as_mut()?;
        let repo = self.repo;

        for obj in objs.iter() {
            match kind {
                Traversal::NthParent(num) => {
                    match self.repo.find_object(*obj).map_err(Error::from).and_then(|obj| {
                        obj.try_into_commit().map_err(|err| {
                            let object::try_into::Error { actual, expected, id } = err;
                            Error::ObjectKind {
                                oid: id.attach(repo).shorten_or_id(),
                                actual,
                                expected,
                            }
                        })
                    }) {
                        Ok(commit) => match commit.parent_ids().skip(num.saturating_sub(1)).next() {
                            Some(id) => replacements.push((commit.id, id.detach())),
                            None => errors.push((
                                commit.id,
                                Error::ParentOutOfRange {
                                    oid: commit.id().shorten_or_id(),
                                    desired: num,
                                    available: commit.parent_ids().count(),
                                },
                            )),
                        },
                        Err(err) => errors.push((*obj, err)),
                    }
                }
                Traversal::NthAncestor(_num) => todo!("ancestor"),
            }
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

    fn peel_until(&mut self, kind: PeelTo<'_>) -> Option<()> {
        self.unset_disambiguate_call();
        self.follow_refs_to_objects_if_needed()?;

        let mut replacements = SmallVec::<[(ObjectId, ObjectId); 1]>::default();
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
            PeelTo::Path(path) => {
                let repo = self.repo;
                let lookup_path = |obj: &ObjectId| {
                    let tree_id = peel(repo, obj, git_object::Kind::Tree)?;
                    let tree = repo.find_object(tree_id)?.into_tree();
                    let entry = tree
                        .lookup_path(git_path::from_bstr(path).components().map(|c| {
                            git_path::os_str_into_bstr(c.as_os_str())
                                .expect("no illformed UTF-8")
                                .as_ref()
                        }))?
                        .ok_or_else(|| Error::PathNotFound {
                            path: path.into(),
                            object: obj.attach(repo).shorten_or_id(),
                            tree: tree_id.attach(repo).shorten_or_id(),
                        })?;
                    Ok(entry.oid)
                };
                for obj in objs.iter() {
                    match lookup_path(obj) {
                        Ok(replace) => replacements.push((*obj, replace)),
                        Err(err) => errors.push((*obj, err)),
                    }
                }
            }
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

    fn find(&mut self, regex: &BStr, negated: bool) -> Option<()> {
        self.unset_disambiguate_call();
        self.follow_refs_to_objects_if_needed()?;

        #[cfg(not(feature = "regex"))]
        let matches = |message: &BStr| -> bool { message.contains_str(regex) ^ negated };
        #[cfg(feature = "regex")]
        let matches = match regex::bytes::Regex::new(regex.to_str_lossy().as_ref()) {
            Ok(compiled) => {
                let needs_regex = regex::escape(compiled.as_str()) != regex;
                move |message: &BStr| -> bool {
                    if needs_regex {
                        compiled.is_match(message) ^ negated
                    } else {
                        message.contains_str(regex) ^ negated
                    }
                }
            }
            Err(err) => {
                self.err.push(err.into());
                return None;
            }
        };

        match self.objs[self.idx].as_mut() {
            Some(objs) => {
                let repo = self.repo;
                let mut errors = Vec::new();
                let mut replacements = SmallVec::<[(ObjectId, ObjectId); 1]>::default();
                for oid in objs.iter() {
                    match oid
                        .attach(repo)
                        .ancestors()
                        .sorting(Sorting::ByCommitTimeNewestFirst)
                        .all()
                    {
                        Ok(iter) => {
                            let mut matched = false;
                            let mut count = 0;
                            let commits = iter.map(|res| {
                                res.map_err(Error::from).and_then(|commit_id| {
                                    commit_id.object().map_err(Error::from).map(|obj| obj.into_commit())
                                })
                            });
                            for commit in commits {
                                count += 1;
                                match commit {
                                    Ok(commit) => {
                                        if matches(commit.message_raw_sloppy()) {
                                            replacements.push((*oid, commit.id));
                                            matched = true;
                                            break;
                                        }
                                    }
                                    Err(err) => errors.push((*oid, err)),
                                }
                            }
                            if !matched {
                                errors.push((
                                    *oid,
                                    Error::NoRegexMatch {
                                        regex: regex.into(),
                                        commits_searched: count,
                                        oid: oid.attach(repo).shorten_or_id(),
                                    },
                                ))
                            }
                        }
                        Err(err) => errors.push((*oid, err.into())),
                    }
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
            None => match self.repo.references() {
                Ok(references) => match references.all() {
                    Ok(references) => {
                        match self
                            .repo
                            .rev_walk(
                                references
                                    .peeled()
                                    .filter_map(Result::ok)
                                    .filter(|r| {
                                        r.id()
                                            .object()
                                            .ok()
                                            .map(|obj| obj.kind == git_object::Kind::Commit)
                                            .unwrap_or(false)
                                    })
                                    .filter_map(|r| r.detach().peeled),
                            )
                            .sorting(Sorting::ByCommitTimeNewestFirst)
                            .all()
                        {
                            Ok(iter) => {
                                let mut matched = false;
                                let mut count = 0;
                                let commits = iter.map(|res| {
                                    res.map_err(Error::from).and_then(|commit_id| {
                                        commit_id.object().map_err(Error::from).map(|obj| obj.into_commit())
                                    })
                                });
                                for commit in commits {
                                    count += 1;
                                    match commit {
                                        Ok(commit) => {
                                            if matches(commit.message_raw_sloppy()) {
                                                self.objs[self.idx]
                                                    .get_or_insert_with(HashSet::default)
                                                    .insert(commit.id);
                                                matched = true;
                                                break;
                                            }
                                        }
                                        Err(err) => self.err.push(err),
                                    }
                                }
                                if matched {
                                    Some(())
                                } else {
                                    self.err.push(Error::NoRegexMatchAllRefs {
                                        regex: regex.into(),
                                        commits_searched: count,
                                    });
                                    None
                                }
                            }
                            Err(err) => {
                                self.err.push(err.into());
                                None
                            }
                        }
                    }
                    Err(err) => {
                        self.err.push(err.into());
                        None
                    }
                },
                Err(err) => {
                    self.err.push(err.into());
                    None
                }
            },
        }
    }

    fn index_lookup(&mut self, path: &BStr, stage: u8) -> Option<()> {
        self.unset_disambiguate_call();
        match self.repo.index() {
            Ok(index) => match index.entry_by_path_and_stage(path, stage.into()) {
                Some(entry) => {
                    self.objs[self.idx]
                        .get_or_insert_with(HashSet::default)
                        .insert(entry.id);
                    Some(())
                }
                None => {
                    let stage_hint = [0, 1, 2]
                        .iter()
                        .filter(|our_stage| **our_stage != stage)
                        .find_map(|stage| {
                            index
                                .entry_index_by_path_and_stage(path, (*stage).into())
                                .map(|_| (*stage).into())
                        });
                    let exists = self
                        .repo
                        .work_dir()
                        .map_or(false, |root| root.join(git_path::from_bstr(path)).exists());
                    self.err.push(Error::IndexLookup {
                        desired_path: path.into(),
                        desired_stage: stage.into(),
                        exists,
                        stage_hint,
                    });
                    None
                }
            },
            Err(err) => {
                self.err.push(err.into());
                None
            }
        }
    }
}

impl<'repo> delegate::Kind for Delegate<'repo> {
    fn kind(&mut self, kind: git_revision::spec::Kind) -> Option<()> {
        use git_revision::spec::Kind::*;
        self.kind = Some(kind);

        if self.kind_implies_committish() {
            self.disambiguate_objects_by_fallback_hint(ObjectKindHint::Committish.into());
        }
        if matches!(kind, RangeBetween | ReachableToMergeBase) {
            self.idx += 1;
        }

        Some(())
    }
}

fn peel(repo: &Repository, obj: &git_hash::oid, kind: git_object::Kind) -> Result<ObjectId, Error> {
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
            oid: obj.id.attach(repo).shorten_or_id(),
        })
    }
}
