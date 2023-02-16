#![no_main]
use gix_revision::spec::parse::{delegate, Delegate};
use libfuzzer_sys::fuzz_target;

use bstr::BStr;

fuzz_target!(|data: &[u8]| {
    drop(gix_revision::spec::parse(data.into(), &mut Noop));
});

struct Noop;

impl Delegate for Noop {
    fn done(&mut self) {}
}

impl delegate::Kind for Noop {
    fn kind(&mut self, _kind: gix_revision::spec::Kind) -> Option<()> {
        Some(())
    }
}

impl delegate::Navigate for Noop {
    fn traverse(&mut self, _kind: delegate::Traversal) -> Option<()> {
        Some(())
    }

    fn peel_until(&mut self, _kind: delegate::PeelTo<'_>) -> Option<()> {
        Some(())
    }

    fn find(&mut self, _regex: &BStr, _negated: bool) -> Option<()> {
        Some(())
    }

    fn index_lookup(&mut self, _path: &BStr, _stage: u8) -> Option<()> {
        Some(())
    }
}

impl delegate::Revision for Noop {
    fn find_ref(&mut self, _name: &BStr) -> Option<()> {
        Some(())
    }

    fn disambiguate_prefix(
        &mut self,
        _prefix: gix_hash::Prefix,
        _hint: Option<delegate::PrefixHint<'_>>,
    ) -> Option<()> {
        Some(())
    }

    fn reflog(&mut self, _query: delegate::ReflogLookup) -> Option<()> {
        Some(())
    }

    fn nth_checked_out_branch(&mut self, _branch_no: usize) -> Option<()> {
        Some(())
    }

    fn sibling_branch(&mut self, _kind: delegate::SiblingBranch) -> Option<()> {
        Some(())
    }
}
