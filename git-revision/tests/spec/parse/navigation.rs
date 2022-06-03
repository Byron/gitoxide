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
        let rec = parse("@^^^10");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD",);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(
            rec.traversal,
            vec![
                Traversal::NthParent(1),
                Traversal::NthParent(1),
                Traversal::NthParent(10)
            ]
        );
        assert_eq!(rec.calls, 4);
    }

    #[test]
    fn followed_by_zero_is_peeling_to_commit() {
        let rec = parse("@^0");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD",);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.traversal.len(), 0, "traversals by parent are never zero");
        assert_eq!(
            rec.peel_to[0],
            PeelTo::ObjectKind(git_object::Kind::Commit),
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
        for (spec, expected_parent) in [
            ("HEAD^1", 1),
            ("abcd^10", 10),
            ("v1.3.4^123", 123),
            ("v1.3.4-12-g1234^1000", 1000),
        ] {
            let rec = parse(spec);

            assert!(rec.kind.is_none());
            assert!(rec.find_ref[0].as_ref().is_some() || rec.prefix[0].is_some());
            assert_eq!(rec.traversal[0], Traversal::NthParent(expected_parent));
            assert_eq!(rec.calls, 2);
        }
    }
}
