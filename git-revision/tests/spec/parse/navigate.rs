mod caret_symbol {
    use crate::spec::parse::{parse, try_parse};
    use git_revision::spec;
    use git_revision::spec::parse::delegate::{PeelTo, Traversal};

    #[test]
    fn single_is_first_parent() {
        let rec = parse("@^");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD",);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.traversal[0], Traversal::NthParent(1));
        assert_eq!(rec.calls, 2);
    }

    #[test]
    fn multiple_calls_stack() {
        let rec = parse("@^^^10^0^{tag}^020");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD",);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(
            rec.traversal,
            vec![
                Traversal::NthParent(1),
                Traversal::NthParent(1),
                Traversal::NthParent(10),
                Traversal::NthParent(20),
            ]
        );
        assert_eq!(
            rec.peel_to,
            vec![
                PeelTo::ObjectKind(git_object::Kind::Commit),
                PeelTo::ObjectKind(git_object::Kind::Tag)
            ]
        );
        assert_eq!(rec.calls, 7);
    }

    #[test]
    fn followed_by_zero_is_peeling_to_commit() {
        let rec = parse("@^0");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD",);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.traversal.len(), 0, "traversals by parent are never zero");
        assert_eq!(
            rec.peel_to,
            vec![PeelTo::ObjectKind(git_object::Kind::Commit)],
            "instead 0 serves as shortcut"
        );
        assert_eq!(rec.calls, 2);
    }

    #[test]
    fn negative_numbers_are_invalid() {
        let err = try_parse("@^-1").unwrap_err();
        assert!(matches!(err, spec::parse::Error::SignedNumber {input} if input == "-1"))
    }

    #[test]
    fn explicitly_positive_numbers_are_invalid() {
        let err = try_parse("@^+1").unwrap_err();
        assert!(matches!(err, spec::parse::Error::SignedNumber {input} if input == "+1"))
    }

    #[test]
    fn explicit_parent_number() {
        for (spec, expected) in [
            ("HEAD^1", 1),
            ("abcd^10", 10),
            ("v1.3.4^123", 123),
            ("v1.3.4-12-g1234^1000", 1000),
        ] {
            let rec = parse(spec);

            assert!(rec.kind.is_none());
            assert!(rec.find_ref[0].as_ref().is_some() || rec.prefix[0].is_some());
            assert_eq!(rec.traversal, vec![Traversal::NthParent(expected)]);
            assert_eq!(rec.calls, 2);
        }
    }

    #[test]
    fn peel_to_object_type() {
        for (spec, expected) in [
            ("HEAD^{commit}", PeelTo::ObjectKind(git_object::Kind::Commit)),
            ("abcd^{tree}", PeelTo::ObjectKind(git_object::Kind::Tree)),
            ("v1.3.4^{blob}", PeelTo::ObjectKind(git_object::Kind::Blob)),
            ("v1.3.4-12-g1234^{tag}", PeelTo::ObjectKind(git_object::Kind::Tag)),
            ("v1.3.4-12-g1234^{object}", PeelTo::ExistingObject),
        ] {
            let rec = parse(spec);

            assert!(rec.kind.is_none());
            assert!(rec.find_ref[0].as_ref().is_some() || rec.prefix[0].is_some());
            assert_eq!(rec.peel_to, vec![expected]);
            assert_eq!(rec.calls, 2);
        }
    }

    #[test]
    #[ignore]
    fn regex_with_revision_starting_point() {
        for (spec, (regex, negated)) in [
            ("HEAD^{/simple}", ("simple", false)),
            ("abcd^{/!-negated}", ("negated", true)),
            ("v1.3.4^{/^from start}", ("^from start", false)),
            (
                "v1.3.4-12-g1234^{/!!leading exclamation mark}",
                ("!leading exclamation mark", false),
            ),
            ("v1.3.4-12-g1234^{with count{1}}", ("with count{1}", false)),
        ] {
            let rec = parse(spec);

            assert!(rec.kind.is_none());
            assert!(rec.find_ref[0].as_ref().is_some() || rec.prefix[0].is_some());
            assert_eq!(rec.patterns, vec![(regex.into(), negated)]);
            assert_eq!(rec.calls, 2);
        }
    }

    #[test]
    fn empty_braces_deref_a_tag() {
        let rec = parse("v1.2^{}");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "v1.2");
        assert_eq!(rec.peel_to, vec![PeelTo::RecursiveTagObject]);
        assert_eq!(rec.calls, 2);
    }

    #[test]
    fn invalid_object_type() {
        let err = try_parse("@^{invalid}").unwrap_err();
        assert!(matches!(err, spec::parse::Error::InvalidObject {input} if input == "invalid"));

        let err = try_parse("@^{Commit}").unwrap_err();
        assert!(
            matches!(err, spec::parse::Error::InvalidObject {input} if input == "Commit"),
            "these types are case sensitive"
        );
    }

    #[test]
    #[ignore]
    fn empty_top_level_regex_are_invalid() {
        // git also can't do it, finds nothing instead. It could be the youngest commit in theory, but isn't.
    }

    #[test]
    #[ignore]
    fn regex_resolution_is_skipped_if_empty_and_if_revision_is_set() {
        // because they always match
    }
}
