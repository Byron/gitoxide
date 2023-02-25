use anyhow::bail;
use gix::{
    bstr::{BStr, BString},
    revision::plumbing::{
        spec,
        spec::parse::{
            delegate,
            delegate::{PeelTo, ReflogLookup, SiblingBranch, Traversal},
            Delegate,
        },
    },
};

pub fn explain(spec: std::ffi::OsString, mut out: impl std::io::Write) -> anyhow::Result<()> {
    let mut explain = Explain::new(&mut out);
    let spec = gix::path::os_str_into_bstr(&spec)?;
    gix::revision::plumbing::spec::parse(spec, &mut explain)?;
    if let Some(err) = explain.err {
        bail!(err);
    }
    Ok(())
}

struct Explain<'a> {
    out: &'a mut dyn std::io::Write,
    call: usize,
    ref_name: Option<BString>,
    oid_prefix: Option<gix::hash::Prefix>,
    has_implicit_anchor: bool,
    err: Option<String>,
}

impl<'a> Explain<'a> {
    fn new(out: &'a mut impl std::io::Write) -> Self {
        Explain {
            out,
            call: 0,
            ref_name: None,
            oid_prefix: None,
            has_implicit_anchor: false,
            err: None,
        }
    }
    fn prefix(&mut self) -> Option<()> {
        self.call += 1;
        write!(self.out, "{:02}. ", self.call).ok()
    }
    fn revision_name(&self) -> BString {
        self.ref_name.clone().unwrap_or_else(|| {
            self.oid_prefix
                .expect("parser must have set some object value")
                .to_string()
                .into()
        })
    }
}

impl<'a> delegate::Revision for Explain<'a> {
    fn find_ref(&mut self, name: &BStr) -> Option<()> {
        self.prefix()?;
        self.ref_name = Some(name.into());
        writeln!(self.out, "Lookup the '{}' reference", name).ok()
    }

    fn disambiguate_prefix(&mut self, prefix: gix::hash::Prefix, hint: Option<delegate::PrefixHint<'_>>) -> Option<()> {
        self.prefix()?;
        self.oid_prefix = Some(prefix);
        writeln!(
            self.out,
            "Disambiguate the '{}' object name ({})",
            prefix,
            match hint {
                None => "any object".to_string(),
                Some(delegate::PrefixHint::MustBeCommit) => "commit".into(),
                Some(delegate::PrefixHint::DescribeAnchor { ref_name, generation }) => format!(
                    "commit {} generations in future of reference {:?}",
                    generation, ref_name
                ),
            }
        )
        .ok()
    }

    fn reflog(&mut self, query: ReflogLookup) -> Option<()> {
        self.prefix()?;
        self.has_implicit_anchor = true;
        let ref_name: &BStr = self
            .ref_name
            .as_ref()
            .map(|n| n.as_ref())
            .unwrap_or_else(|| "HEAD".into());
        match query {
            ReflogLookup::Entry(no) => {
                writeln!(self.out, "Find entry {} in reflog of '{}' reference", no, ref_name).ok()
            }
            ReflogLookup::Date(time) => writeln!(
                self.out,
                "Find entry closest to time {} in reflog of '{}' reference",
                time.format(gix::date::time::format::ISO8601),
                ref_name
            )
            .ok(),
        }
    }

    fn nth_checked_out_branch(&mut self, branch_no: usize) -> Option<()> {
        self.prefix()?;
        self.has_implicit_anchor = true;
        writeln!(self.out, "Find the {}th checked-out branch of 'HEAD'", branch_no).ok()
    }

    fn sibling_branch(&mut self, kind: SiblingBranch) -> Option<()> {
        self.prefix()?;
        self.has_implicit_anchor = true;
        let ref_info = match self.ref_name.as_ref() {
            Some(ref_name) => format!("'{}'", ref_name),
            None => "behind 'HEAD'".into(),
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
        let name = self.revision_name();
        writeln!(
            self.out,
            "{}",
            match kind {
                Traversal::NthAncestor(no) => format!("Traverse to the {}th ancestor of revision named '{}'", no, name),
                Traversal::NthParent(no) => format!("Select the {}th parent of revision named '{}'", no, name),
            }
        )
        .ok()
    }

    fn peel_until(&mut self, kind: PeelTo<'_>) -> Option<()> {
        self.prefix()?;
        writeln!(
            self.out,
            "{}",
            match kind {
                PeelTo::ValidObject => "Assure the current object exists".to_string(),
                PeelTo::RecursiveTagObject => "Follow the current annotated tag until an object is found".into(),
                PeelTo::ObjectKind(kind) => format!("Peel the current object until it is a {}", kind),
                PeelTo::Path(path) => format!("Lookup the object at '{}' from the current tree-ish", path),
            }
        )
        .ok()
    }

    fn find(&mut self, regex: &BStr, negated: bool) -> Option<()> {
        self.prefix()?;
        self.has_implicit_anchor = true;
        let negate_text = if negated { "does not match" } else { "matches" };
        writeln!(
            self.out,
            "{}",
            match self
                .ref_name
                .as_ref()
                .map(|n| n.to_string())
                .or_else(|| self.oid_prefix.map(|p| p.to_string()))
            {
                Some(obj_name) => format!(
                    "Follow the ancestry of revision '{}' until a commit message {} regex '{}'",
                    obj_name, negate_text, regex
                ),
                None => format!(
                    "Find the most recent commit from any reference including 'HEAD' that {} regex '{}'",
                    negate_text, regex
                ),
            }
        )
        .ok()
    }

    fn index_lookup(&mut self, path: &BStr, stage: u8) -> Option<()> {
        self.prefix()?;
        self.has_implicit_anchor = true;
        writeln!(
            self.out,
            "Lookup the index at path '{}' stage {} ({})",
            path,
            stage,
            match stage {
                0 => "base",
                1 => "ours",
                2 => "theirs",
                _ => unreachable!("BUG: parser assures of that"),
            }
        )
        .ok()
    }
}

impl<'a> delegate::Kind for Explain<'a> {
    fn kind(&mut self, kind: spec::Kind) -> Option<()> {
        self.prefix()?;
        self.call = 0;
        writeln!(
            self.out,
            "Set revision specification to {} mode",
            match kind {
                spec::Kind::RangeBetween => "range",
                spec::Kind::ReachableToMergeBase => "merge-base",
                spec::Kind::ExcludeReachable => "exclude",
                spec::Kind::IncludeReachableFromParents => "include parents",
                spec::Kind::ExcludeReachableFromParents => "exclude parents",
                spec::Kind::IncludeReachable =>
                    unreachable!("BUG: 'single' mode is implied but cannot be set explicitly"),
            }
        )
        .ok()
    }
}

impl<'a> Delegate for Explain<'a> {
    fn done(&mut self) {
        if !self.has_implicit_anchor && self.ref_name.is_none() && self.oid_prefix.is_none() {
            self.err = Some("Incomplete specification lacks its anchor, like a reference or object name".into())
        }
    }
}
