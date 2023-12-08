use std::collections::HashSet;

use gix_hash::ObjectId;
use gix_macros::momo;
use gix_revision::spec::parse;

use crate::{bstr::BStr, revision::Spec, Repository};

mod types;
use crate::bstr::BString;
pub use types::{Error, ObjectKindHint, Options, RefsHint};

///
pub mod single {
    use crate::bstr::BString;

    /// The error returned by [`crate::Repository::rev_parse_single()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Parse(#[from] super::Error),
        #[error("revspec {spec:?} did not resolve to a single object")]
        RangedRev { spec: BString },
    }
}

///
pub mod error;

impl<'repo> Spec<'repo> {
    /// Parse `spec` and use information from `repo` to resolve it, using `opts` to learn how to deal with ambiguity.
    ///
    /// Note that it's easier and to use [`repo.rev_parse()`][Repository::rev_parse()] instead.
    #[momo]
    pub fn from_bstr<'a>(spec: impl Into<&'a BStr>, repo: &'repo Repository, opts: Options) -> Result<Self, Error> {
        let mut delegate = Delegate::new(repo, opts);
        match gix_revision::spec::parse(spec.into(), &mut delegate) {
            Err(parse::Error::Delegate) => Err(delegate.into_err()),
            Err(err) => Err(err.into()),
            Ok(()) => delegate.into_rev_spec(),
        }
    }
}

struct Delegate<'repo> {
    refs: [Option<gix_ref::Reference>; 2],
    objs: [Option<HashSet<ObjectId>>; 2],
    /// Path specified like `@:<path>` or `:<path>` for later use when looking up specs.
    /// Note that it terminates spec parsing, so it's either `0` or `1`, never both.
    paths: [Option<(BString, gix_object::tree::EntryMode)>; 2],
    /// The originally encountered ambiguous objects for potential later use in errors.
    ambiguous_objects: [Option<HashSet<ObjectId>>; 2],
    idx: usize,
    kind: Option<gix_revision::spec::Kind>,

    opts: Options,
    err: Vec<Error>,
    /// The ambiguous prefix obtained during a call to `disambiguate_prefix()`.
    prefix: [Option<gix_hash::Prefix>; 2],
    /// If true, we didn't try to do any other transformation which might have helped with disambiguation.
    last_call_was_disambiguate_prefix: [bool; 2],

    repo: &'repo Repository,
}

mod delegate;
