use bstr::BString;
use git_attributes::State;
use git_pathspec::{MagicSignature, Pattern};

#[test]
fn can_parse() {
    let inputs = vec![
        ("some/path", pat("some/path", None, vec![])),
        ("some/*.path", pat("some/*.path", None, vec![])),
        (":/", pat("", Some(MagicSignature::TOP), vec![])),
        (":^", pat("", Some(MagicSignature::EXCLUDE), vec![])),
        (":!", pat("", Some(MagicSignature::EXCLUDE), vec![])),
        (":(top)", pat("", Some(MagicSignature::TOP), vec![])),
        (":(literal)", pat("", Some(MagicSignature::LITERAL), vec![])),
        (":(icase)", pat("", Some(MagicSignature::ICASE), vec![])),
        (":(glob)", pat("", Some(MagicSignature::GLOB), vec![])),
        (":(attr)", pat("", Some(MagicSignature::ATTR), vec![])),
        (
            ":(attr:someAttr)",
            pat("", Some(MagicSignature::ATTR), vec![("someAttr", State::Set)]),
        ),
        (
            ":(attr:!someAttr)",
            pat("", Some(MagicSignature::ATTR), vec![("someAttr", State::Unspecified)]),
        ),
        (
            ":(attr:-someAttr)",
            pat("", Some(MagicSignature::ATTR), vec![("someAttr", State::Unset)]),
        ),
        (
            ":(attr:someAttr=value)",
            pat(
                "",
                Some(MagicSignature::ATTR),
                vec![("someAttr", State::Value("value".into()))],
            ),
        ),
        (
            ":(attr:someAttr anotherAttr)",
            pat(
                "",
                Some(MagicSignature::ATTR),
                vec![("someAttr", State::Set), ("anotherAttr", State::Set)],
            ),
        ),
        (":(exclude)", pat("", Some(MagicSignature::EXCLUDE), vec![])),
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
        (":/:some/path", pat("some/path", Some(MagicSignature::TOP), vec![])),
        (
            ":!(literal)some/*path",
            pat(
                "some/*path",
                Some(MagicSignature::EXCLUDE | MagicSignature::LITERAL),
                vec![],
            ),
        ),
        (":", pat("", None, vec![])),
        (":()", pat("", None, vec![])),
        (":::::", pat("", None, vec![])),
        (
            ":!/!/:",
            pat("", Some(MagicSignature::TOP | MagicSignature::EXCLUDE), vec![]),
        ),
    ];

    for (input, expected) in inputs {
        assert!(is_valid_in_git(input), "This pathspec is invalid in git: {}", input);

        let pattern = git_pathspec::parse(input.as_bytes()).expect("parsing should not fail");
        assert_eq!(pattern, expected, "while checking input: \"{}\"", input);
    }
}

#[test]
fn should_fail_on_whitespace_or_invalid_keywords() {
    use git_pathspec::parse::Error;
    let inputs = vec![
        (
            ":(top, exclude)some/path",
            Error::InvalidSignature {
                found_signature: BString::from(" exclude"),
            },
        ),
        (
            ":( )some/path",
            Error::InvalidSignature {
                found_signature: BString::from(" "),
            },
        ),
        (
            ":(tp)some/path",
            Error::InvalidSignature {
                found_signature: BString::from("tp"),
            },
        ),
        (
            ":(attr:+someAttr)some/path",
            Error::InvalidAttribute(git_attributes::parse::Error::AttributeName {
                line_number: 0,
                attribute: BString::from("+someAttr"),
            }),
        ),
        (
            ":(top",
            Error::MissingClosingParenthesis {
                pathspec: BString::from(":(top"),
            },
        ),
    ];

    for (input, _expected) in inputs {
        assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());

        // TODO: Find a way to do this without `Eq` trait
        // assert_eq!(output.unwrap_err()., expected);
    }
}

fn pat(path: &str, signature: Option<MagicSignature>, attributes: Vec<(&str, State)>) -> Pattern {
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
