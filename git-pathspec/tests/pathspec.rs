use git_attributes::State;
use git_pathspec::{MagicSignature, Pattern, SearchMode};

mod succeed {
    use crate::{check_valid_inputs, pat, pat_with_path, pat_with_path_and_sig};
    use git_attributes::State;
    use git_pathspec::{MagicSignature, SearchMode};

    #[test]
    fn repeated_matcher_keywords() {
        let input = vec![
            (
                ":(glob,glob)",
                pat("", MagicSignature::empty(), SearchMode::PathAwareGlob, vec![]),
            ),
            (
                ":(literal,literal)",
                pat("", MagicSignature::empty(), SearchMode::Literal, vec![]),
            ),
            (
                ":(top,top)",
                pat("", MagicSignature::TOP, SearchMode::ShellGlob, vec![]),
            ),
            (
                ":(icase,icase)",
                pat("", MagicSignature::ICASE, SearchMode::ShellGlob, vec![]),
            ),
            (
                ":(attr,attr)",
                pat("", MagicSignature::ATTR, SearchMode::ShellGlob, vec![]),
            ),
            (
                ":!^(exclude,exclude)",
                pat("", MagicSignature::EXCLUDE, SearchMode::ShellGlob, vec![]),
            ),
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
            (":(top)", pat_with_path_and_sig("", MagicSignature::TOP)),
            (":(icase)", pat_with_path_and_sig("", MagicSignature::ICASE)),
            (":(attr)", pat_with_path_and_sig("", MagicSignature::ATTR)),
            (":(exclude)", pat_with_path_and_sig("", MagicSignature::EXCLUDE)),
            (
                ":(literal)",
                pat("", MagicSignature::empty(), SearchMode::Literal, vec![]),
            ),
            (
                ":(glob)",
                pat("", MagicSignature::empty(), SearchMode::PathAwareGlob, vec![]),
            ),
            (
                ":(top,exclude)",
                pat_with_path_and_sig("", MagicSignature::TOP | MagicSignature::EXCLUDE),
            ),
            (
                ":(icase,literal)",
                pat("", MagicSignature::ICASE, SearchMode::Literal, vec![]),
            ),
            (
                ":!(literal)some/*path",
                pat("some/*path", MagicSignature::EXCLUDE, SearchMode::Literal, vec![]),
            ),
            (
                ":(top,literal,icase,attr,exclude)some/path",
                pat("some/path", MagicSignature::all(), SearchMode::Literal, vec![]),
            ),
            (
                ":(top,glob,icase,attr,exclude)some/path",
                pat("some/path", MagicSignature::all(), SearchMode::PathAwareGlob, vec![]),
            ),
        ];

        check_valid_inputs(inputs);
    }

    #[test]
    fn attributes_in_signature() {
        let inputs = vec![
            (
                ":(attr:someAttr)",
                pat(
                    "",
                    MagicSignature::ATTR,
                    SearchMode::ShellGlob,
                    vec![("someAttr", State::Set)],
                ),
            ),
            (
                ":(attr:!someAttr)",
                pat(
                    "",
                    MagicSignature::ATTR,
                    SearchMode::ShellGlob,
                    vec![("someAttr", State::Unspecified)],
                ),
            ),
            (
                ":(attr:-someAttr)",
                pat(
                    "",
                    MagicSignature::ATTR,
                    SearchMode::ShellGlob,
                    vec![("someAttr", State::Unset)],
                ),
            ),
            (
                ":(attr:someAttr=value)",
                pat(
                    "",
                    MagicSignature::ATTR,
                    SearchMode::ShellGlob,
                    vec![("someAttr", State::Value("value".into()))],
                ),
            ),
            (
                ":(attr:someAttr anotherAttr)",
                pat(
                    "",
                    MagicSignature::ATTR,
                    SearchMode::ShellGlob,
                    vec![("someAttr", State::Set), ("anotherAttr", State::Set)],
                ),
            ),
        ];

        check_valid_inputs(inputs)
    }

    #[test]
    #[ignore]
    fn attributes_with_escaped_values() {
        let inputs = vec![(
            r":(attr:value=one\,two\,three)",
            pat(
                "",
                MagicSignature::ATTR,
                SearchMode::ShellGlob,
                vec![("value", State::Value("one,two,three".into()))],
            ),
        )];

        check_valid_inputs(inputs)
    }

    #[test]
    #[ignore]
    // TODO: Needs research - what does 'prefix:' do
    fn prefix() {
        let inputs = vec![(
            r":(prefix:)",
            pat(
                "",
                MagicSignature::ATTR,
                SearchMode::ShellGlob,
                vec![("value", State::Value("one,two,three".into()))],
            ),
        )];

        check_valid_inputs(inputs)
    }
}

mod fail {
    use crate::is_valid_in_git;
    use git_pathspec::parse::Error;

    #[test]
    fn empty_input() {
        let input = "";

        assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), Error::EmptyString));
    }

    #[test]
    fn invalid_short_signatures() {
        let inputs = vec![
            ":\"()", ":#()", ":%()", ":&()", ":'()", ":,()", ":-()", ":;()", ":<()", ":=()", ":>()", ":@()", ":_()",
            ":`()", ":~()",
        ];

        inputs.into_iter().for_each(|input| {
            assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

            let output = git_pathspec::parse(input.as_bytes());
            assert!(output.is_err());
            assert!(matches!(output.unwrap_err(), Error::Unimplemented { .. }));
        });
    }

    #[test]
    fn invalid_keywords() {
        let inputs = vec![
            ":( )some/path",
            ":(tp)some/path",
            ":(top, exclude)some/path",
            ":(top,exclude,icse)some/path",
        ];

        inputs.into_iter().for_each(|input| {
            assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

            let output = git_pathspec::parse(input.as_bytes());
            assert!(output.is_err());
            assert!(matches!(output.unwrap_err(), Error::InvalidKeyword { .. }));
        });
    }

    #[test]
    fn invalid_attributes() {
        let inputs = vec![
            ":(attr:+invalidAttr)some/path",
            ":(attr:validAttr +invalidAttr)some/path",
            ":(attr:+invalidAttr,attr:valid)some/path",
            ":(attr:inva\\lid)some/path",
        ];

        for input in inputs {
            assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

            let output = git_pathspec::parse(input.as_bytes());
            assert!(output.is_err());
            assert!(matches!(output.unwrap_err(), Error::InvalidAttribute { .. }));
        }
    }

    #[test]
    fn empty_attribute() {
        let input = ":(attr:)";

        assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), Error::EmptyAttribute));
    }

    #[test]
    fn missing_parentheses() {
        let input = ":(top";

        assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), Error::MissingClosingParenthesis { .. }));
    }

    #[test]
    fn glob_and_literal_keywords_present() {
        let input = ":(glob,literal)some/path";

        assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), Error::IncompatibleSearchModes));
    }

    #[test]
    fn multiple_attribute_specifications() {
        let input = ":(attr:one,attr:two)some/path";

        assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), Error::MultipleAttributeSpecifications));
    }
}

fn check_valid_inputs(inputs: Vec<(&str, Pattern)>) {
    inputs.into_iter().for_each(|(input, expected)| {
        assert!(is_valid_in_git(input), "This pathspec is invalid in git: {}", input);

        let pattern = git_pathspec::parse(input.as_bytes()).expect("parsing should not fail");
        assert_eq!(pattern, expected, "while checking input: \"{}\"", input);
    });
}

fn pat_with_path(path: &str) -> Pattern {
    pat_with_path_and_sig(path, MagicSignature::empty())
}

fn pat_with_path_and_sig(path: &str, signature: MagicSignature) -> Pattern {
    pat(path, signature, SearchMode::ShellGlob, vec![])
}

fn pat(path: &str, signature: MagicSignature, search_mode: SearchMode, attributes: Vec<(&str, State)>) -> Pattern {
    Pattern {
        path: path.into(),
        signature,
        search_mode,
        attributes: attributes
            .into_iter()
            .map(|(attr, state)| (attr.into(), state))
            .collect(),
    }
}

// TODO: Cache results instead of running them with each test run
fn is_valid_in_git(pathspec: &str) -> bool {
    use std::process::Command;

    let output = Command::new("git")
        .args(["ls-files", pathspec])
        .output()
        .expect("failed to execute process");

    output.status.success()
}
