use std::collections::HashSet;

use git_hash::ObjectId;
use git_revision::spec::parse::{
    delegate,
    delegate::{ReflogLookup, SiblingBranch},
};

use crate::{
    bstr::{BStr, BString, ByteSlice},
    ext::ReferenceExt,
    revision::spec::parse::{Delegate, Error, RefsHint},
};

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

        let empty_tree_id = git_hash::ObjectId::empty_tree(prefix.as_oid().kind());
        let res = if prefix.as_oid() == empty_tree_id {
            candidates.as_mut().expect("set").insert(empty_tree_id);
            Ok(Some(Err(())))
        } else {
            self.repo.objects.lookup_prefix(prefix, candidates.as_mut())
        };

        match res {
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

    fn reflog(&mut self, query: ReflogLookup) -> Option<()> {
        self.unset_disambiguate_call();
        match query {
            ReflogLookup::Date(_date) => {
                self.err.push(Error::Planned {
                    dependency: "remote handling and ref-specs are fleshed out more",
                });
                None
            }
            ReflogLookup::Entry(no) => {
                let r = match &mut self.refs[self.idx] {
                    Some(r) => r.clone().attach(self.repo),
                    val @ None => match self.repo.head().map(|head| head.try_into_referent()) {
                        Ok(Some(r)) => {
                            *val = Some(r.clone().detach());
                            r
                        }
                        Ok(None) => {
                            self.err.push(Error::UnbornHeadsHaveNoRefLog);
                            return None;
                        }
                        Err(err) => {
                            self.err.push(err.into());
                            return None;
                        }
                    },
                };
                let mut platform = r.log_iter();
                match platform.rev().ok().flatten() {
                    Some(mut it) => match it.nth(no).and_then(Result::ok) {
                        Some(line) => {
                            self.objs[self.idx]
                                .get_or_insert_with(HashSet::default)
                                .insert(line.new_oid);
                            Some(())
                        }
                        None => {
                            let available = platform.rev().ok().flatten().map_or(0, |it| it.count());
                            self.err.push(Error::RefLogEntryOutOfRange {
                                reference: r.detach(),
                                desired: no,
                                available,
                            });
                            None
                        }
                    },
                    None => {
                        self.err.push(Error::MissingRefLog {
                            reference: r.name().as_bstr().into(),
                            action: "lookup entry",
                        });
                        None
                    }
                }
            }
        }
    }

    fn nth_checked_out_branch(&mut self, branch_no: usize) -> Option<()> {
        self.unset_disambiguate_call();
        fn prior_checkouts_iter<'a>(
            platform: &'a mut git_ref::file::log::iter::Platform<'static, '_>,
        ) -> Result<impl Iterator<Item = (BString, ObjectId)> + 'a, Error> {
            match platform.rev().ok().flatten() {
                Some(log) => Ok(log.filter_map(Result::ok).filter_map(|line| {
                    line.message
                        .strip_prefix(b"checkout: moving from ")
                        .and_then(|from_to| from_to.find(" to ").map(|pos| &from_to[..pos]))
                        .map(|from_branch| (from_branch.into(), line.previous_oid))
                })),
                None => Err(Error::MissingRefLog {
                    reference: "HEAD".into(),
                    action: "search prior checked out branch",
                }),
            }
        }

        let head = match self.repo.head() {
            Ok(head) => head,
            Err(err) => {
                self.err.push(err.into());
                return None;
            }
        };
        match prior_checkouts_iter(&mut head.log_iter()).map(|mut it| it.nth(branch_no.saturating_sub(1))) {
            Ok(Some((ref_name, id))) => {
                let id = match self.repo.find_reference(ref_name.as_bstr()) {
                    Ok(mut r) => {
                        let id = r.peel_to_id_in_place().map(|id| id.detach()).unwrap_or(id);
                        self.refs[self.idx] = Some(r.detach());
                        id
                    }
                    Err(_) => id,
                };
                self.objs[self.idx].get_or_insert_with(HashSet::default).insert(id);
                Some(())
            }
            Ok(None) => {
                self.err.push(Error::PriorCheckoutOutOfRange {
                    desired: branch_no,
                    available: prior_checkouts_iter(&mut head.log_iter())
                        .map(|it| it.count())
                        .unwrap_or(0),
                });
                None
            }
            Err(err) => {
                self.err.push(err);
                None
            }
        }
    }

    fn sibling_branch(&mut self, _kind: SiblingBranch) -> Option<()> {
        self.unset_disambiguate_call();
        self.err.push(Error::Planned {
            dependency: "remote handling and ref-specs are fleshed out more",
        });
        None
    }
}
