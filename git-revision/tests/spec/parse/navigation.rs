use crate::spec::parse::try_parse;
use git_revision::spec;

#[test]
fn braces_must_be_closed() {
    for unclosed_spec in ["@{something", "@{", "@{..@"] {
        let err = try_parse(unclosed_spec).unwrap_err();
        assert!(matches!(err, spec::parse::Error::UnclosedBracePair {input} if input == unclosed_spec[1..]))
    }
}

mod at {
    use crate::spec::parse::{parse, try_parse};
    use git_revision::spec;

    #[test]
    fn reflog_current_branch() {
        for (spec, expected_entry) in [("@{0}", 0), ("@{42}", 42), ("@{00100}", 100)] {
            let rec = parse(spec);

            assert!(rec.kind.is_none());
            assert_eq!(rec.find_ref[0], None,);
            assert_eq!(
                rec.prefix[0], None,
                "neither ref nor prefixes are set, straight to navigation"
            );
            assert_eq!(rec.current_branch_reflog_entry[0], Some(expected_entry));
            assert_eq!(rec.calls, 1);
        }
    }

    #[test]
    fn nth_checked_out_branch() {
        for (spec, expected_branch) in [("@{-1}", 1), ("@{-42}", 42), ("@{-00100}", 100)] {
            let rec = parse(spec);

            assert!(rec.kind.is_none());
            assert_eq!(rec.find_ref[0], None,);
            assert_eq!(
                rec.prefix[0], None,
                "neither ref nor prefixes are set, straight to navigation"
            );
            assert_eq!(rec.nth_checked_out_branch[0], Some(expected_branch));
            assert_eq!(rec.calls, 1);
        }

        let err = try_parse("@{-0}").unwrap_err();
        assert!(
            matches!(err, spec::parse::Error::NegativeZero {input} if input == "-0"),
            "negative zero is not accepted, even though it could easily be defaulted to 0 which is a valid value"
        );
    }
}
