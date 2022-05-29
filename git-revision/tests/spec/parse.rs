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

impl spec::parse::Delegate for Recorder {
    fn set_ref(&mut self, input: &BStr) -> Option<()> {
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

    fn set_prefix(&mut self, input: git_hash::Prefix) -> Option<()> {
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

    fn nth_ancestor(&mut self, _n: usize) -> Option<()> {
        todo!()
    }

    fn nth_parent(&mut self, _n: usize) -> Option<()> {
        todo!()
    }

    fn kind(&mut self, kind: spec::Kind) -> Option<()> {
        self.calls += 1;
        if self.opts.reject_kind {
            return None;
        }
        self.kind = Some(kind);
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
    for spec in ["", " ", "\n\t"] {
        let rec = parse(spec);
        assert_eq!(rec.calls, 1);
    }
}

#[test]
fn all_characters_are_taken_verbatim_which_includes_whitespace() {
    let spec = "  HEAD \n";
    let rec = parse(spec);
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input.unwrap(), spec);
}

mod revision {
    use crate::spec::parse::{parse, try_parse, try_parse_opts, Options};
    use git_revision::spec;

    #[test]
    fn short_hash_likes_are_considered_prefixes() {
        let rec = parse("abCD");
        assert!(rec.kind.is_none());
        assert_eq!(
            rec.resolve_ref_input, None,
            "references are not resolved if prefix lookups succeed"
        );
        assert_eq!(rec.prefix, Some(git_hash::Prefix::from_hex("abcd").unwrap()));
        assert_eq!(rec.calls, 1);

        let rec = parse("gabcd123");
        assert!(rec.kind.is_none());
        assert_eq!(
            rec.resolve_ref_input.unwrap(),
            "gabcd123",
            "ref lookups are performed if it doesn't look like a hex sha"
        );
        assert_eq!(
            rec.prefix, None,
            "prefix lookups are not attempted at all (and they are impossible even)"
        );
        assert_eq!(rec.calls, 1);
    }

    #[test]
    fn unresolvable_hash_likes_are_resolved_as_refs() {
        let rec = try_parse_opts(
            "abCD",
            Options {
                reject_prefix: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(rec.kind.is_none());
        assert_eq!(rec.resolve_ref_input.unwrap(), "abCD");
        assert_eq!(rec.prefix, None);
        assert_eq!(rec.calls, 2);
    }

    #[test]
    fn hash_likes_that_are_too_long_are_resolved_as_refs() {
        let spec = "abcd123456789abcd123456789abcd123456789abcd123456789abcd123456789abcd123456789abcd123456789";
        let rec = try_parse_opts(
            spec,
            Options {
                reject_prefix: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(rec.kind.is_none());
        assert_eq!(rec.resolve_ref_input.unwrap(), spec);
        assert_eq!(rec.prefix, None);
        assert_eq!(
            rec.calls, 1,
            "we can't create a prefix from it, hence only ref resolution is attempted"
        );
    }

    #[test]
    fn at_by_iteself_is_shortcut_for_head() {
        let rec = parse("@");
        assert!(rec.kind.is_none());
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }

    #[test]
    fn multiple_ats_are_invalid_but_may_cause_callbacks() {
        let err = try_parse("@@").unwrap_err();
        assert!(matches!(err, spec::parse::Error::UnconsumedInput {input} if input == "@"));
    }

    #[test]
    fn lonely_at_after_ref_is_invalid() {
        let err = try_parse("HEAD@").unwrap_err();
        assert!(matches!(err, spec::parse::Error::AtNeedsCurlyBrackets {input} if input == ""));
    }

    #[test]
    fn refname_head() {
        let rec = parse("HEAD");
        assert!(rec.kind.is_none());
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }

    #[test]
    fn refname_with_head_prefix() {
        let rec = parse("HEADfake");
        assert!(rec.kind.is_none());
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEADfake");
    }

    #[test]
    fn full_head_ref_name() {
        let rec = parse("refs/heads/main");
        assert!(rec.kind.is_none());
        assert_eq!(rec.resolve_ref_input.unwrap(), "refs/heads/main");
    }
}

mod range {
    use crate::spec::parse::{parse, try_parse_opts, Options};
    use git_revision::spec;

    #[test]
    fn delegate_can_refuse_spec_kinds() {
        let err = try_parse_opts(
            "^HEAD",
            Options {
                reject_kind: true,
                ..Default::default()
            },
        )
        .unwrap_err();
        assert!(
            matches!(err, spec::parse::Error::Delegate),
            "Delegates can refuse spec kind changes to abort parsing early"
        );
    }

    #[test]
    fn leading_caret_is_range_kind() {
        let rec = parse("^HEAD");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }

    #[test]
    fn trailing_dot_dot_is_range() {
        let rec = parse("HEAD..");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }

    #[test]
    fn trailing_dot_dot_dot_is_merge_base() {
        let rec = parse("HEAD...");
        assert_eq!(rec.kind.unwrap(), spec::Kind::MergeBase);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }

    #[test]
    fn middle_dot_dot_dot_is_merge_base() {
        let rec = parse("HEAD...@");
        assert_eq!(rec.kind.unwrap(), spec::Kind::MergeBase);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
        assert_eq!(rec.resolve_ref_input2.unwrap(), "HEAD");
    }

    #[test]
    fn middle_dot_dot_is_range() {
        let rec = parse("@..HEAD");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
        assert_eq!(rec.resolve_ref_input2.unwrap(), "HEAD");
    }
}
