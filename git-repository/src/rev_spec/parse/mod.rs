use crate::bstr::BStr;
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
        let mut delegate = Delegate::new(repo, opts);
        match git_revision::spec::parse(spec.into(), &mut delegate) {
            Err(parse::Error::Delegate) => Err(delegate.into_err()),
            Err(err) => Err(err.into()),
            Ok(()) => delegate.into_rev_spec(),
        }
    }
}

struct Delegate<'repo> {
    refs: [Option<git_ref::Reference>; 2],
    objs: [Option<HashSet<ObjectId>>; 2],
    /// The originally encountered ambiguous objects for potential later use in errors.
    ambiguous_objects: [Option<HashSet<ObjectId>>; 2],
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
