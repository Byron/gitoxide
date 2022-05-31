use git_attributes::State;
use git_pathspec::parse::Error;
use git_pathspec::{MagicSignature, Pattern};

#[test]
fn can_parse() {
    let inputs = vec![
        ("some/path", pat("some/path", MagicSignature::empty(), vec![])),
        ("some/*.path", pat("some/*.path", MagicSignature::empty(), vec![])),
        (":/", pat("", MagicSignature::TOP, vec![])),
        (":^", pat("", MagicSignature::EXCLUDE, vec![])),
        (":!", pat("", MagicSignature::EXCLUDE, vec![])),
        (":(top)", pat("", MagicSignature::TOP, vec![])),
        (":(literal)", pat("", MagicSignature::LITERAL, vec![])),
        (":(icase)", pat("", MagicSignature::ICASE, vec![])),
        (":(glob)", pat("", MagicSignature::GLOB, vec![])),
        (":(attr)", pat("", MagicSignature::ATTR, vec![])),
        (
            ":(attr:someAttr)",
            pat("", MagicSignature::ATTR, vec![("someAttr", State::Set)]),
        ),
        (
            ":(attr:!someAttr)",
            pat("", MagicSignature::ATTR, vec![("someAttr", State::Unspecified)]),
        ),
        (
            ":(attr:-someAttr)",
            pat("", MagicSignature::ATTR, vec![("someAttr", State::Unset)]),
        ),
        (
            ":(attr:someAttr=value)",
            pat(
                "",
                MagicSignature::ATTR,
                vec![("someAttr", State::Value("value".into()))],
            ),
        ),
        (
            ":(attr:someAttr anotherAttr)",
            pat(
                "",
                MagicSignature::ATTR,
                vec![("someAttr", State::Set), ("anotherAttr", State::Set)],
            ),
        ),
        (":(exclude)", pat("", MagicSignature::EXCLUDE, vec![])),
        // TODO:
        // 'literal' and 'glob' cannot appear in the same pathspec together
        // is this the parsers job to handle?
        // (
        //     ":(top,literal,icase,glob,attr,exclude)some/path",
        //     pat(
        //         "some/path",
        //         Some(
        //             MagicSignature::TOP
        //                 | MagicSignature::LITERAL
        //                 | MagicSignature::ICASE
        //                 | MagicSignature::GLOB
        //                 | MagicSignature::ATTR
        //                 | MagicSignature::EXCLUDE,
        //         ),
        //         vec![]
        //     ),
        // ),
        (":/:some/path", pat("some/path", MagicSignature::TOP, vec![])),
        (
            ":!(literal)some/*path",
            pat("some/*path", MagicSignature::EXCLUDE | MagicSignature::LITERAL, vec![]),
        ),
        (":", pat("", MagicSignature::empty(), vec![])),
        (":()", pat("", MagicSignature::empty(), vec![])),
        (":::::", pat("", MagicSignature::empty(), vec![])),
        (":!/!/:", pat("", MagicSignature::TOP | MagicSignature::EXCLUDE, vec![])),
    ];

    for (input, expected) in inputs {
        assert!(is_valid_in_git(input), "This pathspec is invalid in git: {}", input);

        let pattern = git_pathspec::parse(input.as_bytes()).expect("parsing should not fail");
        assert_eq!(pattern, expected, "while checking input: \"{}\"", input);
    }
}

#[test]
fn should_fail_on_invalid_keywords() {
    let inputs = vec![":( )some/path", ":(tp)some/path", ":(top, exclude)some/path"];

    for input in inputs {
        assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), Error::InvalidKeyword { .. }));
    }
}

#[test]
fn should_fail_on_invalid_attributes() {
    let inputs = vec![
        ":(attr:+invalidAttr)some/path",
        ":(attr:validAttr +invalidAttr)some/path",
    ];

    for input in inputs {
        assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), Error::InvalidAttribute { .. }));
    }
}

#[test]
fn should_fail_on_missing_parentheses() {
    let inputs = vec![":(top"];

    for input in inputs {
        assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), Error::MissingClosingParenthesis { .. }));
    }
}

fn pat(path: &str, signature: MagicSignature, attributes: Vec<(&str, State)>) -> Pattern {
    Pattern {
        path: path.into(),
        signature,
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
