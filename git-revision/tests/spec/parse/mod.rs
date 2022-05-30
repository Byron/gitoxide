use git_object::bstr::{BStr, BString};
use git_revision::spec;

#[derive(Default, Debug)]
struct Options {
    reject_kind: bool,
    reject_prefix: bool,
}

#[derive(Default, Debug)]
struct Recorder {
    resolve_ref_input: Option<BString>,
    resolve_ref_input2: Option<BString>,
    prefix: Option<git_hash::Prefix>,
    prefix2: Option<git_hash::Prefix>,
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
}

impl spec::parse::delegate::Anchor for Recorder {
    fn find_ref(&mut self, input: &BStr) -> Option<()> {
        if self.resolve_ref_input.is_none() {
            self.resolve_ref_input = input.to_owned().into();
        } else if self.resolve_ref_input2.is_none() {
            self.resolve_ref_input2 = input.to_owned().into();
        } else {
            panic!("called resolve_ref more than twice with '{}'", input);
        }
        self.calls += 1;
        Some(())
    }

    fn disambiguate_prefix(&mut self, input: git_hash::Prefix) -> Option<()> {
        self.calls += 1;
        if self.opts.reject_prefix {
            return None;
        }
        if self.prefix.is_none() {
            self.prefix = input.into();
        } else if self.prefix2.is_none() {
            self.prefix2 = input.into();
        } else {
            panic!("called find_by_prefix more than twice with '{}'", input);
        }
        Some(())
    }
}

impl spec::parse::delegate::Navigation for Recorder {
    fn nth_ancestor(&mut self, _n: usize) -> Option<()> {
        todo!()
    }

    fn nth_parent(&mut self, _n: usize) -> Option<()> {
        todo!()
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
    assert_eq!(rec.resolve_ref_input.unwrap(), spec);
}

mod revision;

mod range;
