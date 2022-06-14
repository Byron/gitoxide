#![allow(unused_variables)]

use git::bstr::{BStr, BString, ByteSlice};
use git::revision::spec::parse::{delegate, Delegate};
use git::revision::{
    spec,
    spec::parse::delegate::{PeelTo, ReflogLookup, SiblingBranch, Traversal},
};
use git_repository as git;
use std::ffi::OsString;

struct Explain<'a> {
    out: &'a mut dyn std::io::Write,
    call: usize,
    ref_name: Option<BString>,
    oid_prefix: Option<git::hash::Prefix>,
}

impl<'a> Explain<'a> {
    fn new(out: &'a mut impl std::io::Write) -> Self {
        Explain {
            out,
            call: 0,
            ref_name: None,
            oid_prefix: None,
        }
    }
    fn prefix(&mut self) -> Option<()> {
        self.call += 1;
        write!(self.out, "{:02}. ", self.call).ok()
    }
}

impl<'a> delegate::Revision for Explain<'a> {
    fn find_ref(&mut self, name: &BStr) -> Option<()> {
        self.prefix()?;
        writeln!(self.out, "Lookup the '{}' reference", name).ok()
    }

    fn disambiguate_prefix(&mut self, prefix: git::hash::Prefix) -> Option<()> {
        self.prefix()?;
        self.oid_prefix = Some(prefix);
        writeln!(self.out, "Disambiguate the '{}' object name", prefix).ok()
    }

    fn reflog(&mut self, query: ReflogLookup) -> Option<()> {
        self.prefix()?;
        let ref_name: &BStr = self.ref_name.as_ref().map(|n| n.as_ref()).unwrap_or("HEAD".into());
        match query {
            ReflogLookup::Entry(no) => {
                writeln!(self.out, "Find entry {} in reflog of '{}' reference", no, ref_name).ok()
            }
            ReflogLookup::Date(time) => {
                let mut buf = Vec::new();
                time.write_to(&mut buf).ok()?;
                writeln!(
                    self.out,
                    "Find entry closest to time {} in reflog of '{}' reference",
                    buf.as_bstr(),
                    ref_name
                )
                .ok()
            }
        }
    }

    fn nth_checked_out_branch(&mut self, branch_no: usize) -> Option<()> {
        self.prefix()?;
        writeln!(self.out, "Find the {}th checked-out branch of 'HEAD'", branch_no).ok()
    }

    fn sibling_branch(&mut self, kind: SiblingBranch) -> Option<()> {
        self.prefix()?;
        let ref_name: &BStr = self.ref_name.as_ref().map(|n| n.as_ref()).unwrap_or("HEAD".into());
        let ref_info = match self.ref_name.as_ref() {
            Some(ref_name) => format!("'{}'", ref_name),
            None => format!("behind 'HEAD'"),
        };
        writeln!(
            self.out,
            "Lookup the remote '{}' branch of local reference {}",
            match kind {
                SiblingBranch::Upstream => "upstream",
                SiblingBranch::Push => "push",
            },
            ref_info
        )
        .ok()
    }
}

impl<'a> delegate::Navigate for Explain<'a> {
    fn traverse(&mut self, kind: Traversal) -> Option<()> {
        self.prefix()?;
        todo!()
    }

    fn peel_until(&mut self, kind: PeelTo<'_>) -> Option<()> {
        self.prefix()?;
        todo!()
    }

    fn find(&mut self, regex: &BStr, negated: bool) -> Option<()> {
        self.prefix()?;
        todo!()
    }

    fn index_lookup(&mut self, path: &BStr, stage: u8) -> Option<()> {
        self.prefix()?;
        todo!()
    }
}

impl<'a> delegate::Kind for Explain<'a> {
    fn kind(&mut self, kind: spec::Kind) -> Option<()> {
        self.prefix()?;
        todo!()
    }
}

impl<'a> Delegate for Explain<'a> {
    fn done(&mut self) {}
}

pub fn explain(_repo: git::Repository, spec: OsString, mut out: impl std::io::Write) -> anyhow::Result<()> {
    let mut explain = Explain::new(&mut out);
    let spec = git::path::os_str_into_bstr(&spec)?;
    git::revision::spec::parse(spec, &mut explain).map_err(anyhow::Error::from)
}
