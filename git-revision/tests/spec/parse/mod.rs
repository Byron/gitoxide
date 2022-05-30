use git_object::bstr::{BStr, BString};
use git_revision::spec;
use std::fmt::Display;

#[derive(Default, Debug)]
struct Options {
    reject_kind: bool,
    reject_prefix: bool,
}

#[derive(Default, Debug)]
struct Recorder {
    // anchors
    find_ref: [Option<BString>; 2],
    prefix: [Option<git_hash::Prefix>; 2],

    // navigation
    current_branch_reflog_entry: [Option<usize>; 2],

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

fn set_val<T: Display>(fn_name: &str, store: &mut [Option<T>; 2], val: T) -> Option<()> {
    for entry in store.iter_mut() {
        if entry.is_none() {
            *entry = Some(val);
            return Some(());
        }
    }
    panic!("called {}() more than twice with '{}'", fn_name, val);
}

impl spec::parse::delegate::Anchor for Recorder {
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
}

impl spec::parse::delegate::Navigation for Recorder {
    fn current_branch_reflog(&mut self, entry: usize) -> Option<()> {
        self.calls += 1;
        set_val("current_branch_reflog", &mut self.current_branch_reflog_entry, entry)
    }
}

impl spec::parse::delegate::Kind for Recorder {
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
mod navigation;
