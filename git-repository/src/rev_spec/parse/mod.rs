use crate::bstr::BStr;
use crate::types::RevSpecDetached;
use crate::Repository;
use crate::RevSpec;
use git_hash::ObjectId;
use git_revision::spec::parse;
use std::collections::HashSet;

mod types;
pub use types::{Error, ObjectKindHint, Options, RefsHint};

///
pub mod error;

impl<'repo> RevSpec<'repo> {
    /// Parse `spec` and use information from `repo` to resolve it, using `opts` to learn how to deal with ambiguity.
    pub fn from_bstr<'a>(spec: impl Into<&'a BStr>, repo: &'repo Repository, opts: Options) -> Result<Self, Error> {
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
            Err(parse::Error::Delegate) => {
                return Err(Error::from_errors(delegate.err));
            }
            Err(err) => return Err(err.into()),
            Ok(()) => {
                let range = zero_or_one_objects_or_ambguity_err(delegate.objs, delegate.prefix, delegate.err, repo)?;
                RevSpec {
                    inner: RevSpecDetached {
                        from_ref: delegate.refs[0].take(),
                        from: range[0],
                        to_ref: delegate.refs[1].take(),
                        to: range[1],
                        kind: delegate.kind,
                    },
                    repo,
                }
            }
        };
        Ok(spec)
    }
}

struct Delegate<'repo> {
    refs: [Option<git_ref::Reference>; 2],
    objs: [Option<HashSet<ObjectId>>; 2],
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

mod delegate;
