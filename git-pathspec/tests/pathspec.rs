pub use git_testtools::Result;

mod parse {
    use std::collections::HashMap;

    use bstr::{BStr, BString, ByteSlice};
    use git_attributes::State;
    use git_pathspec::{MagicSignature, MatchMode, Pattern};
    use once_cell::sync::Lazy;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct PatternForTesting {
        path: BString,
        signature: MagicSignature,
        search_mode: MatchMode,
        attributes: Vec<(BString, State)>,
    }

    impl From<Pattern> for PatternForTesting {
        fn from(p: Pattern) -> Self {
            PatternForTesting {
                path: p.path,
                signature: p.signature,
                search_mode: p.search_mode,
                attributes: p
                    .attributes
                    .into_iter()
                    .map(|attr| (attr.name.as_str().into(), attr.state))
                    .collect(),
            }
        }
    }

    static BASELINE: Lazy<HashMap<BString, usize>> = Lazy::new(|| {
        let base = git_testtools::scripted_fixture_read_only("generate_pathspec_baseline.sh").unwrap();

        (|| -> crate::Result<_> {
            let mut map = HashMap::new();
            let baseline = std::fs::read(base.join("baseline.git"))?;
            let mut lines = baseline.lines();
            while let Some(spec) = lines.next() {
                let exit_code = lines.next().expect("two lines per baseline").to_str()?.parse()?;
                map.insert(spec.into(), exit_code);
            }
            Ok(map)
        })()
        .unwrap()
    });

    #[test]
    fn baseline() {
        for (pattern, exit_code) in BASELINE.iter() {
            let res = git_pathspec::parse(pattern);
            assert_eq!(
                res.is_ok(),
                *exit_code == 0,
                "{pattern:?} disagrees with baseline: {res:?}"
            )
        }
    }

    mod succeed {
        use git_attributes::State;
        use git_pathspec::{MagicSignature, MatchMode};

        use crate::parse::{
            check_valid_inputs, pat, pat_with_attrs, pat_with_path, pat_with_path_and_sig, pat_with_search_mode,
            pat_with_sig,
        };

        #[test]
        fn there_is_no_pathspec_pathspec() {
            check_valid_inputs(Some((":", pat_with_attrs(vec![]))));
        }

        #[test]
        fn repeated_matcher_keywords() {
            let input = vec![
                (":(glob,glob)", pat_with_search_mode(MatchMode::PathAwareGlob)),
                (":(literal,literal)", pat_with_search_mode(MatchMode::Literal)),
                (":(top,top)", pat_with_sig(MagicSignature::TOP)),
                (":(icase,icase)", pat_with_sig(MagicSignature::ICASE)),
                (":(attr,attr)", pat_with_attrs(vec![])),
                (":!^(exclude,exclude)", pat_with_sig(MagicSignature::EXCLUDE)),
            ];

            check_valid_inputs(input);
        }

        #[test]
        fn empty_signatures() {
            let inputs = vec![
                (".", pat_with_path(".")),
                ("some/path", pat_with_path("some/path")),
                (":some/path", pat_with_path("some/path")),
                (":()some/path", pat_with_path("some/path")),
                ("::some/path", pat_with_path("some/path")),
                (":::some/path", pat_with_path(":some/path")),
                (":():some/path", pat_with_path(":some/path")),
            ];

            check_valid_inputs(inputs)
        }

        #[test]
        fn whitespace_in_pathspec() {
            let inputs = vec![
                (" some/path", pat_with_path(" some/path")),
                ("some/ path", pat_with_path("some/ path")),
                ("some/path ", pat_with_path("some/path ")),
                (": some/path", pat_with_path(" some/path")),
                (": !some/path", pat_with_path(" !some/path")),
                (": :some/path", pat_with_path(" :some/path")),
                (": ()some/path", pat_with_path(" ()some/path")),
                (
                    ":! some/path",
                    pat_with_path_and_sig(" some/path", MagicSignature::EXCLUDE),
                ),
            ];

            check_valid_inputs(inputs)
        }

        #[test]
        fn short_signatures() {
            let inputs = vec![
                (":/some/path", pat_with_path_and_sig("some/path", MagicSignature::TOP)),
                (
                    ":^some/path",
                    pat_with_path_and_sig("some/path", MagicSignature::EXCLUDE),
                ),
                (
                    ":!some/path",
                    pat_with_path_and_sig("some/path", MagicSignature::EXCLUDE),
                ),
                (
                    ":/!some/path",
                    pat_with_path_and_sig("some/path", MagicSignature::TOP | MagicSignature::EXCLUDE),
                ),
                (
                    ":!/^/:some/path",
                    pat_with_path_and_sig("some/path", MagicSignature::TOP | MagicSignature::EXCLUDE),
                ),
            ];

            check_valid_inputs(inputs)
        }

        #[test]
        fn signatures_and_searchmodes() {
            let inputs = vec![
                (":(top)", pat_with_sig(MagicSignature::TOP)),
                (":(icase)", pat_with_sig(MagicSignature::ICASE)),
                (":(attr)", pat_with_path("")),
                (":(exclude)", pat_with_sig(MagicSignature::EXCLUDE)),
                (":(literal)", pat_with_search_mode(MatchMode::Literal)),
                (":(glob)", pat_with_search_mode(MatchMode::PathAwareGlob)),
                (
                    ":(top,exclude)",
                    pat_with_sig(MagicSignature::TOP | MagicSignature::EXCLUDE),
                ),
                (
                    ":(icase,literal)",
                    pat("", MagicSignature::ICASE, MatchMode::Literal, vec![]),
                ),
                (
                    ":!(literal)some/*path",
                    pat("some/*path", MagicSignature::EXCLUDE, MatchMode::Literal, vec![]),
                ),
                (
                    ":(top,literal,icase,attr,exclude)some/path",
                    pat("some/path", MagicSignature::all(), MatchMode::Literal, vec![]),
                ),
                (
                    ":(top,glob,icase,attr,exclude)some/path",
                    pat("some/path", MagicSignature::all(), MatchMode::PathAwareGlob, vec![]),
                ),
            ];

            check_valid_inputs(inputs);
        }

        #[test]
        fn attributes_in_signature() {
            let inputs = vec![
                (":(attr:someAttr)", pat_with_attrs(vec![("someAttr", State::Set)])),
                (
                    ":(attr:!someAttr)",
                    pat_with_attrs(vec![("someAttr", State::Unspecified)]),
                ),
                (":(attr:-someAttr)", pat_with_attrs(vec![("someAttr", State::Unset)])),
                (
                    ":(attr:someAttr=value)",
                    pat_with_attrs(vec![("someAttr", State::Value("value".into()))]),
                ),
                (
                    ":(attr:a=one b=)",
                    pat_with_attrs(vec![("a", State::Value("one".into())), ("b", State::Value("".into()))]),
                ),
                (
                    ":(attr:a= b=two)",
                    pat_with_attrs(vec![("a", State::Value("".into())), ("b", State::Value("two".into()))]),
                ),
                (
                    ":(attr:a=one b=two)",
                    pat_with_attrs(vec![
                        ("a", State::Value("one".into())),
                        ("b", State::Value("two".into())),
                    ]),
                ),
                (
                    ":(attr:a=one   b=two)",
                    pat_with_attrs(vec![
                        ("a", State::Value("one".into())),
                        ("b", State::Value("two".into())),
                    ]),
                ),
                (
                    ":(attr:someAttr anotherAttr)",
                    pat_with_attrs(vec![("someAttr", State::Set), ("anotherAttr", State::Set)]),
                ),
            ];

            check_valid_inputs(inputs)
        }

        #[test]
        fn attributes_with_escape_chars_in_state_values() {
            let inputs = vec![
                (
                    r":(attr:v=one\-)",
                    pat_with_attrs(vec![("v", State::Value(r"one-".into()))]),
                ),
                (
                    r":(attr:v=one\_)",
                    pat_with_attrs(vec![("v", State::Value(r"one_".into()))]),
                ),
                (
                    r":(attr:v=one\,)",
                    pat_with_attrs(vec![("v", State::Value(r"one,".into()))]),
                ),
                (
                    r":(attr:v=one\,two\,three)",
                    pat_with_attrs(vec![("v", State::Value(r"one,two,three".into()))]),
                ),
                (
                    r":(attr:a=\d b= c=\d)",
                    pat_with_attrs(vec![
                        ("a", State::Value(r"d".into())),
                        ("b", State::Value(r"".into())),
                        ("c", State::Value(r"d".into())),
                    ]),
                ),
            ];

            check_valid_inputs(inputs)
        }
    }

    mod fail {
        use git_pathspec::parse::Error;

        use crate::parse::check_against_baseline;

        #[test]
        fn empty_input() {
            let input = "";

            assert!(
                !check_against_baseline(input),
                "This pathspec is valid in git: {}",
                input
            );

            let output = git_pathspec::parse(input.as_bytes());
            assert!(output.is_err());
            assert!(matches!(output.unwrap_err(), Error::EmptyString));
        }

        #[test]
        fn invalid_short_signatures() {
            let inputs = vec![
                ":\"()", ":#()", ":%()", ":&()", ":'()", ":,()", ":-()", ":;()", ":<()", ":=()", ":>()", ":@()",
                ":_()", ":`()", ":~()",
            ];

            for input in inputs.into_iter() {
                assert!(
                    !check_against_baseline(input),
                    "This pathspec is valid in git: {}",
                    input
                );

                let output = git_pathspec::parse(input.as_bytes());
                assert!(output.is_err());
                assert!(matches!(output.unwrap_err(), Error::Unimplemented { .. }));
            }
        }

        #[test]
        fn invalid_keywords() {
            let inputs = vec![
                ":( )some/path",
                ":(tp)some/path",
                ":(top, exclude)some/path",
                ":(top,exclude,icse)some/path",
            ];

            for input in inputs.into_iter() {
                assert!(
                    !check_against_baseline(input),
                    "This pathspec is valid in git: {}",
                    input
                );

                let output = git_pathspec::parse(input.as_bytes());
                assert!(output.is_err());
                assert!(matches!(output.unwrap_err(), Error::InvalidKeyword { .. }));
            }
        }

        #[test]
        fn invalid_attributes() {
            let inputs = vec![
                ":(attr:+invalidAttr)some/path",
                ":(attr:validAttr +invalidAttr)some/path",
                ":(attr:+invalidAttr,attr:valid)some/path",
                r":(attr:inva\lid)some/path",
            ];

            for input in inputs {
                assert!(
                    !check_against_baseline(input),
                    "This pathspec is valid in git: {}",
                    input
                );

                let output = git_pathspec::parse(input.as_bytes());
                assert!(output.is_err(), "This pathspec did not produce an error {}", input);
                assert!(matches!(output.unwrap_err(), Error::InvalidAttribute { .. }));
            }
        }

        #[test]
        fn invalid_attribute_values() {
            let inputs = vec![
                r":(attr:v=inva#lid)some/path",
                r":(attr:v=inva\\lid)some/path",
                r":(attr:v=invalid\\)some/path",
                r":(attr:v=invalid\#)some/path",
                r":(attr:v=inva\=lid)some/path",
                r":(attr:a=valid b=inva\#lid)some/path",
                ":(attr:v=val��)",
                ":(attr:pr=pre��x:,)�",
            ];

            for input in inputs {
                assert!(
                    !check_against_baseline(input),
                    "This pathspec is valid in git: {}",
                    input
                );

                let output = git_pathspec::parse(input.as_bytes());
                assert!(output.is_err(), "This pathspec did not produce an error {}", input);
                assert!(
                    matches!(output.unwrap_err(), Error::InvalidAttributeValue { .. }),
                    "Errors did not match for pathspec: {}",
                    input
                );
            }
        }

        #[test]
        fn escape_character_at_end_of_attribute_value() {
            let inputs = vec![
                r":(attr:v=invalid\)some/path",
                r":(attr:v=invalid\ )some/path",
                r":(attr:v=invalid\ valid)some/path",
            ];

            for input in inputs {
                assert!(
                    !check_against_baseline(input),
                    "This pathspec is valid in git: {}",
                    input
                );

                let output = git_pathspec::parse(input.as_bytes());
                assert!(output.is_err(), "This pathspec did not produce an error {}", input);
                assert!(matches!(output.unwrap_err(), Error::TrailingEscapeCharacter));
            }
        }

        #[test]
        fn empty_attribute_specification() {
            let input = ":(attr:)";

            assert!(
                !check_against_baseline(input),
                "This pathspec is valid in git: {}",
                input
            );

            let output = git_pathspec::parse(input.as_bytes());
            assert!(output.is_err());
            assert!(matches!(output.unwrap_err(), Error::EmptyAttribute));
        }

        #[test]
        fn multiple_attribute_specifications() {
            let input = ":(attr:one,attr:two)some/path";

            assert!(
                !check_against_baseline(input),
                "This pathspec is valid in git: {}",
                input
            );

            let output = git_pathspec::parse(input.as_bytes());
            assert!(output.is_err());
            assert!(matches!(output.unwrap_err(), Error::MultipleAttributeSpecifications));
        }

        #[test]
        fn missing_parentheses() {
            let input = ":(top";

            assert!(
                !check_against_baseline(input),
                "This pathspec is valid in git: {}",
                input
            );

            let output = git_pathspec::parse(input.as_bytes());
            assert!(output.is_err());
            assert!(matches!(output.unwrap_err(), Error::MissingClosingParenthesis { .. }));
        }

        #[test]
        fn glob_and_literal_keywords_present() {
            let input = ":(glob,literal)some/path";

            assert!(
                !check_against_baseline(input),
                "This pathspec is valid in git: {}",
                input
            );

            let output = git_pathspec::parse(input.as_bytes());
            assert!(output.is_err());
            assert!(matches!(output.unwrap_err(), Error::IncompatibleSearchModes));
        }
    }

    fn check_valid_inputs<'a>(inputs: impl IntoIterator<Item = (&'a str, PatternForTesting)>) {
        for (input, expected) in inputs.into_iter() {
            assert!(
                check_against_baseline(input),
                "This pathspec is invalid in git: {}",
                input
            );

            let pattern: PatternForTesting = git_pathspec::parse(input.as_bytes())
                .unwrap_or_else(|_| panic!("parsing should not fail with pathspec {}", input))
                .into();
            assert_eq!(pattern, expected, "while checking input: \"{}\"", input);
        }
    }

    fn pat_with_path(path: &str) -> PatternForTesting {
        pat_with_path_and_sig(path, MagicSignature::empty())
    }

    fn pat_with_path_and_sig(path: &str, signature: MagicSignature) -> PatternForTesting {
        pat(path, signature, MatchMode::ShellGlob, vec![])
    }

    fn pat_with_sig(signature: MagicSignature) -> PatternForTesting {
        pat("", signature, MatchMode::ShellGlob, vec![])
    }

    fn pat_with_attrs(attrs: Vec<(&'static str, State)>) -> PatternForTesting {
        pat("", MagicSignature::empty(), MatchMode::ShellGlob, attrs)
    }

    fn pat_with_search_mode(search_mode: MatchMode) -> PatternForTesting {
        pat("", MagicSignature::empty(), search_mode, vec![])
    }

    fn pat(
        path: &str,
        signature: MagicSignature,
        search_mode: MatchMode,
        attributes: Vec<(&str, State)>,
    ) -> PatternForTesting {
        PatternForTesting {
            path: path.into(),
            signature,
            search_mode,
            attributes: attributes
                .into_iter()
                .map(|(attr, state)| (attr.into(), state))
                .collect(),
        }
    }

    fn check_against_baseline(pathspec: &str) -> bool {
        let key: &BStr = pathspec.into();
        let base = BASELINE
            .get(key)
            .unwrap_or_else(|| panic!("missing baseline for pathspec: {:?}", pathspec));
        *base == 0
    }
}
