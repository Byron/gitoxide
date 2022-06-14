use git::revision::spec::parse::{delegate, Delegate};
use git_repository as git;
use git_repository::bstr::BStr;
use git_repository::hash::Prefix;
use git_repository::revision::spec::parse::delegate::{PeelTo, ReflogLookup, SiblingBranch, Traversal};
use git_repository::revision::spec::Kind;
use std::ffi::OsString;

struct Explain<'a> {
    out: &'a mut dyn std::io::Write,
}

impl<'a> delegate::Revision for Explain<'a> {
    fn find_ref(&mut self, name: &BStr) -> Option<()> {
        todo!()
    }

    fn disambiguate_prefix(&mut self, prefix: Prefix) -> Option<()> {
        todo!()
    }

    fn reflog(&mut self, query: ReflogLookup) -> Option<()> {
        todo!()
    }

    fn nth_checked_out_branch(&mut self, branch_no: usize) -> Option<()> {
        todo!()
    }

    fn sibling_branch(&mut self, kind: SiblingBranch) -> Option<()> {
        todo!()
    }
}

impl<'a> delegate::Navigate for Explain<'a> {
    fn traverse(&mut self, kind: Traversal) -> Option<()> {
        todo!()
    }

    fn peel_until(&mut self, kind: PeelTo<'_>) -> Option<()> {
        todo!()
    }

    fn find(&mut self, regex: &BStr, negated: bool) -> Option<()> {
        todo!()
    }

    fn index_lookup(&mut self, path: &BStr, stage: u8) -> Option<()> {
        todo!()
    }
}

impl<'a> delegate::Kind for Explain<'a> {
    fn kind(&mut self, kind: Kind) -> Option<()> {
        todo!()
    }
}

impl<'a> Delegate for Explain<'a> {
    fn done(&mut self) {
        todo!()
    }
}

pub fn explain(_repo: git::Repository, spec: OsString, mut out: impl std::io::Write) -> anyhow::Result<()> {
    let mut explain = Explain { out: &mut out };
    let spec = git::path::os_str_into_bstr(&spec)?;
    git::revision::spec::parse(spec, &mut explain).map_err(anyhow::Error::from)
}
