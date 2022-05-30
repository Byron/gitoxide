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
    use crate::spec::parse::parse;

    #[test]
    fn reflog_current_branch() {
        for (spec, expected_entry) in [("@{0}", 0), ("@{42}", 42), ("@{00100}", 100)] {
            let rec = parse(spec);

            assert!(rec.kind.is_none());
            assert_eq!(rec.find_ref[0], None,);
            assert_eq!(
                rec.prefix[0], None,
                "neither ref nor prefixes are set, straight ot navigation"
            );
            assert_eq!(rec.current_branch_reflog_entry[0], Some(expected_entry));
            assert_eq!(rec.calls, 1);
        }
    }
}
