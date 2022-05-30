use crate::spec::parse::{try_parse, try_parse_opts, Options};
use git_revision::spec;

#[test]
fn cannot_declare_ranges_multiple_times() {
    for invalid_spec in ["^HEAD..", "^HEAD..."] {
        let err = try_parse(invalid_spec).unwrap_err();
        assert!(matches!(err, spec::parse::Error::KindSetTwice { .. }));
    }
}

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
        "Delegates can refuse spec kind changes to abort parsing early in case they want single-specs only"
    );
}

mod range {
    use crate::spec::parse::kind::prefix;
    use crate::spec::parse::parse;
    use git_revision::spec;

    #[test]
    fn leading_caret() {
        let rec = parse("^HEAD");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
        assert_eq!(rec.prefix, None);
        assert_eq!(rec.calls, 2);

        let rec = parse("^abcd");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input, None);
        assert_eq!(rec.prefix, prefix("abcd").into());
        assert_eq!(rec.calls, 2);

        let rec = parse("^r1");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "r1");
        assert_eq!(rec.prefix, None);
        assert_eq!(rec.calls, 2);

        let rec = parse("^hello-0-gabcd-dirty");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input, None);
        assert_eq!(rec.prefix, prefix("abcd").into());
        assert_eq!(rec.calls, 2);
    }

    #[test]
    fn trailing_dot_dot() {
        let rec = parse("HEAD..");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
        assert_eq!(rec.prefix, None);
        assert_eq!(rec.calls, 2);
    }

    #[test]
    fn middle_dot_dot() {
        let rec = parse("@..HEAD");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
        assert_eq!(rec.resolve_ref_input2.unwrap(), "HEAD");
        assert_eq!(rec.calls, 3);

        let rec = parse("r1..r2");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "r1");
        assert_eq!(rec.resolve_ref_input2.unwrap(), "r2");
        assert_eq!(rec.calls, 3);

        let rec = parse("abcd..1234");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.prefix, prefix("abcd").into());
        assert_eq!(rec.prefix2, prefix("1234").into());
        assert_eq!(rec.calls, 3);

        let rec = parse("r1..abcd");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "r1");
        assert_eq!(rec.prefix, prefix("abcd").into());
        assert_eq!(rec.calls, 3);

        let rec = parse("abcd-dirty..v1.2-42-g1234");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input, None);
        assert_eq!(rec.prefix, prefix("abcd").into());
        assert_eq!(rec.prefix2, prefix("1234").into());
        assert_eq!(rec.calls, 3);

        let rec = parse("v1.2-42-g1234..abcd-dirty");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input, None);
        assert_eq!(rec.prefix, prefix("1234").into());
        assert_eq!(rec.prefix2, prefix("abcd").into());
        assert_eq!(rec.calls, 3);
    }
}

mod mergebase {
    use crate::spec::parse::kind::prefix;
    use crate::spec::parse::parse;
    use git_revision::spec;

    #[test]
    fn trailing_dot_dot_dot() {
        let rec = parse("HEAD...");
        assert_eq!(rec.kind.unwrap(), spec::Kind::MergeBase);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
        assert_eq!(rec.prefix, None);
        assert_eq!(rec.calls, 2);
    }

    #[test]
    fn middle_dot_dot_dot() {
        let rec = parse("HEAD...@");
        assert_eq!(rec.kind.unwrap(), spec::Kind::MergeBase);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
        assert_eq!(rec.resolve_ref_input2.unwrap(), "HEAD");
        assert_eq!(rec.calls, 3);

        let rec = parse("r1...abcd");
        assert_eq!(rec.kind.unwrap(), spec::Kind::MergeBase);
        assert_eq!(rec.resolve_ref_input.unwrap(), "r1");
        assert_eq!(rec.prefix, prefix("abcd").into());
        assert_eq!(rec.calls, 3);

        let rec = parse("v1.2.3-beta.1-42-g1234-dirty...abcd-dirty");
        assert_eq!(rec.kind.unwrap(), spec::Kind::MergeBase);
        assert_eq!(rec.resolve_ref_input, None);
        assert_eq!(rec.prefix, prefix("1234").into());
        assert_eq!(rec.prefix2, prefix("abcd").into());
        assert_eq!(rec.calls, 3);
    }
}

fn prefix(hex: &str) -> git_hash::Prefix {
    git_hash::Prefix::from_hex(hex).unwrap()
}
