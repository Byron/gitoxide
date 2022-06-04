use git_object::bstr::{BStr, BString};
use git_revision::spec;
use git_revision::spec::parse::delegate;

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Options {
    reject_kind: bool,
    reject_prefix: bool,
}

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Recorder {
    // anchors
    find_ref: [Option<BString>; 2],
    prefix: [Option<git_hash::Prefix>; 2],
    current_branch_reflog_entry: [Option<String>; 2],
    nth_checked_out_branch: [Option<usize>; 2],
    sibling_branch: [Option<String>; 2],

    // navigation
    traversal: Vec<delegate::Traversal>,
    peel_to: Vec<delegate::PeelTo>,

    // range
    kind: Option<spec::Kind>,

    calls: usize,
    opts: Options,
}

impl Recorder {
    fn with(options: Options) -> Self {
        Recorder {
            opts: options,
            ..Default::default()
        }
    }

    fn get_ref(&self, idx: usize) -> &BStr {
        self.find_ref[idx].as_ref().map(|b| b.as_ref()).unwrap()
    }
}

fn set_val<T: std::fmt::Debug>(fn_name: &str, store: &mut [Option<T>; 2], val: T) -> Option<()> {
    for entry in store.iter_mut() {
        if entry.is_none() {
            *entry = Some(val);
            return Some(());
        }
    }
    panic!("called {}() more than twice with '{:?}'", fn_name, val);
}

impl delegate::Revision for Recorder {
    fn find_ref(&mut self, input: &BStr) -> Option<()> {
        self.calls += 1;
        set_val("find_ref", &mut self.find_ref, input.into())
    }

    fn disambiguate_prefix(&mut self, input: git_hash::Prefix) -> Option<()> {
        self.calls += 1;
        if self.opts.reject_prefix {
            return None;
        }
        set_val("disambiguate_prefix", &mut self.prefix, input)
    }

    fn reflog(&mut self, entry: delegate::ReflogLookup) -> Option<()> {
        self.calls += 1;
        set_val(
            "current_branch_reflog",
            &mut self.current_branch_reflog_entry,
            match entry {
                delegate::ReflogLookup::Entry(no) => no.to_string(),
                delegate::ReflogLookup::Date(time) => {
                    let mut buf = Vec::new();
                    time.write_to(&mut buf).unwrap();
                    BString::from(buf).to_string()
                }
            },
        )
    }

    fn nth_checked_out_branch(&mut self, branch: usize) -> Option<()> {
        assert_ne!(branch, 0);
        self.calls += 1;
        set_val("nth_checked_out_branch", &mut self.nth_checked_out_branch, branch)
    }

    fn sibling_branch(&mut self, kind: delegate::SiblingBranch) -> Option<()> {
        self.calls += 1;
        set_val("sibling_branch", &mut self.sibling_branch, format!("{:?}", kind))
    }
}

impl delegate::Navigate for Recorder {
    fn traverse(&mut self, kind: delegate::Traversal) -> Option<()> {
        self.calls += 1;
        self.traversal.push(kind);
        Some(())
    }

    fn peel_until(&mut self, kind: delegate::PeelTo) -> Option<()> {
        self.calls += 1;
        self.peel_to.push(kind);
        Some(())
    }
}

impl delegate::Kind for Recorder {
    fn kind(&mut self, kind: spec::Kind) -> Option<()> {
        self.calls += 1;
        if self.opts.reject_kind {
            return None;
        }
        if self.kind.is_none() {
            self.kind = Some(kind);
        } else {
            panic!("called kind more than once with '{:?}'", kind);
        }
        Some(())
    }
}

fn parse(spec: &str) -> Recorder {
    try_parse_opts(spec, Options::default()).unwrap()
}

fn try_parse(spec: &str) -> Result<Recorder, spec::parse::Error> {
    try_parse_opts(spec, Default::default())
}

fn try_parse_opts(spec: &str, options: Options) -> Result<Recorder, spec::parse::Error> {
    let mut rec = Recorder::with(options);
    spec::parse(spec.into(), &mut rec)?;
    Ok(rec)
}

#[test]
fn empty_specs_are_valid() {
    // they should of course be invalid for the delegate. CLIs may pre-process the input as well if they wish
    // but git itself doesn't do that.
    for spec in [" ", "\n\t"] {
        let rec = parse(spec);
        assert_eq!(rec.calls, 1);
    }
    let rec = parse("");
    assert_eq!(rec.calls, 0, "but we do not bother to call the delegate with nothing");
}

#[test]
fn all_characters_are_taken_verbatim_which_includes_whitespace() {
    let spec = "  HEAD \n";
    let rec = parse(spec);
    assert!(rec.kind.is_none());
    assert_eq!(rec.get_ref(0), spec);
}

mod anchor;
mod kind;
mod navigate;
