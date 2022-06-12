mod colon_symbol {
    use crate::spec::parse::{parse, PeelToOwned as PeelTo};
    use git_revision::spec::parse::delegate::Traversal;

    #[test]
    fn paths_consume_all_remaining_input_as_they_refer_to_blobs() {
        let rec = parse("@:../relative/path...@^^~~");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD");
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.traversal.len(), 0);
        assert_eq!(rec.peel_to, vec![PeelTo::Path("../relative/path...@^^~~".into())]);
        assert_eq!(rec.calls, 2);

        let rec = parse("@:absolute/path^{object}");
        assert_eq!(
            rec.peel_to,
            vec![PeelTo::Path("absolute/path^{object}".into())],
            "this includes useful navigation like object-existence, a shortcoming git shares, proper implementation needs escaping as well."
        );

        let rec = parse("@:absolute/path^{tree}");
        assert_eq!(
            rec.peel_to,
            vec![PeelTo::Path("absolute/path^{tree}".into())],
            "this includes useful navigation like assertion of trees/blobs, we may make this possible in future but for now are as open as git"
        );
    }

    #[test]
    fn empty_paths_refer_to_the_root_tree() {
        let rec = parse("@:");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD");
        assert_eq!(rec.peel_to, vec![PeelTo::Path("".into())]);
        assert_eq!(rec.calls, 2);
    }

    #[test]
    fn paths_have_to_be_last_but_stack_with_other_navigation() {
        let rec = parse("HEAD@{1}~10^2^{commit}:README.md");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD");
        assert_eq!(rec.current_branch_reflog_entry[0], Some("1".to_string()));
        assert_eq!(rec.traversal, vec![Traversal::NthAncestor(10), Traversal::NthParent(2)]);
        assert_eq!(
            rec.peel_to,
            vec![
                PeelTo::ObjectKind(git_object::Kind::Commit),
                PeelTo::Path("README.md".into())
            ]
        );
        assert_eq!(rec.calls, 6);
    }
}

mod tilde_symbol {
    use crate::spec::parse::parse;
    use git_revision::spec::parse::delegate::Traversal;

    #[test]
    fn single_is_first_ancestor() {
        let rec = parse("@~");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD",);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.traversal[0], Traversal::NthAncestor(1));
        assert_eq!(rec.calls, 2);
    }

    #[test]
    fn followed_by_zero_is_no_op() {
        let rec = parse("@~0");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD",);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.calls, 1);
    }

    #[test]
    fn multiple_calls_stack() {
        let rec = parse("@~~~10~0~020");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD",);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(
            rec.traversal,
            vec![
                Traversal::NthAncestor(1),
                Traversal::NthAncestor(1),
                Traversal::NthAncestor(10),
                Traversal::NthAncestor(20),
            ]
        );
        assert_eq!(rec.calls, 5);
    }
}

mod caret_symbol {
    use crate::spec::parse::PeelToOwned as PeelTo;
    use crate::spec::parse::{parse, try_parse};
    use git_revision::spec;
    use git_revision::spec::parse::delegate::Traversal;

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
    fn regex_backslash_rules() {
        for (spec, regex, msg) in [
            (
                r#"@^{/with count{1}}"#,
                r#"with count{1}"#,
                "matching inner parens do not need escaping",
            ),
            (
                r#"@^{/with count\{1\}}"#,
                r#"with count{1}"#,
                "escaped parens are entirely ignored",
            ),
            (r#"@^{/1\}}"#, r#"1}"#, "unmatched closing parens need to be escaped"),
            (r#"@^{/2\{}"#, r#"2{"#, "unmatched opening parens need to be escaped"),
            (
                r#"@^{/3{\{}}"#,
                r#"3{{}"#,
                "unmatched nested opening parens need to be escaped",
            ),
            (
                r#"@^{/4{\}}}"#,
                r#"4{}}"#,
                "unmatched nested closing parens need to be escaped",
            ),
            (
                r#"@^{/a\b\c}"#,
                r#"a\b\c"#,
                "single backslashes do not need to be escaped",
            ),
            (
                r#"@^{/a\b\c\\}"#,
                r#"a\b\c\"#,
                "single backslashes do not need to be escaped, trailing",
            ),
            (
                r#"@^{/a\\b\\c\\}"#,
                r#"a\b\c\"#,
                "backslashes can be escaped nonetheless, trailing",
            ),
            (
                r#"@^{/5\\{}}"#,
                r#"5\{}"#,
                "backslashes in front of parens must be escaped or they would unbalance the brace pair",
            ),
        ] {
            let rec = try_parse(spec).expect(msg);

            assert!(rec.kind.is_none());
            assert!(rec.find_ref[0].as_ref().is_some() || rec.prefix[0].is_some());
            assert_eq!(rec.patterns, vec![(regex.into(), false)], "{}", msg);
            assert_eq!(rec.calls, 2);
        }
    }

    #[test]
    fn regex_with_revision_starting_point_and_negation() {
        for (spec, (regex, negated)) in [
            ("HEAD^{/simple}", ("simple", false)),
            ("abcd^{/!-negated}", ("negated", true)),
            ("v1.3.4^{/^from start}", ("^from start", false)),
            (
                "v1.3.4-12-g1234^{/!!leading exclamation mark}",
                ("!leading exclamation mark", false),
            ),
            ("v1.3.4-12-g1234^{/with count{1}}", ("with count{1}", false)),
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
    fn incomplete_escaped_braces_in_regex_are_invalid() {
        let err = try_parse(r#"@^{/a\{1}}"#).unwrap_err();
        assert!(matches!(err, spec::parse::Error::UnconsumedInput {input} if input == "}"));

        let err = try_parse(r#"@^{/a{1\}}"#).unwrap_err();
        assert!(matches!(err, spec::parse::Error::UnclosedBracePair {input} if input == r#"{/a{1\}}"#));
    }

    #[test]
    fn regex_with_empty_exclamation_mark_prefix_is_invalid() {
        let err = try_parse(r#"@^{/!hello}"#).unwrap_err();
        assert!(matches!(err, spec::parse::Error::UnspecifiedRegexModifier {regex} if regex == "/!hello"));
    }

    #[test]
    fn bad_escapes_can_cause_brace_mismatch() {
        let err = try_parse(r#"@^{\}"#).unwrap_err();
        assert!(matches!(err, spec::parse::Error::UnclosedBracePair {input} if input == r#"{\}"#));

        let err = try_parse(r#"@^{{\}}"#).unwrap_err();
        assert!(matches!(err, spec::parse::Error::UnclosedBracePair {input} if input == r#"{{\}}"#));
    }

    #[test]
    fn empty_top_revision_regex_are_skipped_as_they_match_everything() {
        let rec = parse("@^{/}");

        assert!(rec.kind.is_none());
        assert_eq!(rec.get_ref(0), "HEAD");
        assert!(
            rec.patterns.is_empty(),
            "The delegate won't be called with empty regexes"
        );
        assert_eq!(rec.calls, 1);
    }
}
