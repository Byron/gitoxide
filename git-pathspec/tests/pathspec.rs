use git_attributes::State;
use git_pathspec::parse::Error;
use git_pathspec::{MagicSignature, Pattern};

#[test]
fn can_parse_empty_signatures() {
    let inputs = vec![
        ("some/path", pat("some/path", MagicSignature::empty(), vec![])),
        (":some/path", pat("some/path", MagicSignature::empty(), vec![])),
        (":()some/path", pat("some/path", MagicSignature::empty(), vec![])),
        ("::some/path", pat("some/path", MagicSignature::empty(), vec![])),
        (":::some/path", pat(":some/path", MagicSignature::empty(), vec![])),
    ];

    check_valid_inputs(inputs)
}

#[test]
fn can_parse_short_signatures() {
    let inputs = vec![
        (":/some/path", pat("some/path", MagicSignature::TOP, vec![])),
        (":^some/path", pat("some/path", MagicSignature::EXCLUDE, vec![])),
        (":!some/path", pat("some/path", MagicSignature::EXCLUDE, vec![])),
        (
            ":/!some/path",
            pat("some/path", MagicSignature::TOP | MagicSignature::EXCLUDE, vec![]),
        ),
        (":!/^/:", pat("", MagicSignature::TOP | MagicSignature::EXCLUDE, vec![])),
    ];

    check_valid_inputs(inputs)
}

#[test]
fn can_parse_signatures() {
    let inputs = vec![
        (":(top)", pat("", MagicSignature::TOP, vec![])),
        (":(literal)", pat("", MagicSignature::LITERAL, vec![])),
        (":(icase)", pat("", MagicSignature::ICASE, vec![])),
        (":(glob)", pat("", MagicSignature::GLOB, vec![])),
        (":(attr)", pat("", MagicSignature::ATTR, vec![])),
        (":(exclude)", pat("", MagicSignature::EXCLUDE, vec![])),
        (
            ":(top,exclude)",
            pat("", MagicSignature::TOP | MagicSignature::EXCLUDE, vec![]),
        ),
        (
            ":(icase,literal)",
            pat("", MagicSignature::ICASE | MagicSignature::LITERAL, vec![]),
        ),
        (
            ":!(literal)some/*path",
            pat("some/*path", MagicSignature::EXCLUDE | MagicSignature::LITERAL, vec![]),
        ),
        // TODO:
        // 'literal' and 'glob' cannot appear in the same pathspec together
        // adjust Pattern struct to properly represent this case
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
    ];

    check_valid_inputs(inputs);
}

#[test]
fn can_parse_attributes_in_signature() {
    let inputs = vec![
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
    ];

    check_valid_inputs(inputs)
}

#[test]
fn should_fail_on_empty_input() {
    let input = "";

    assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

    let output = git_pathspec::parse(input.as_bytes());
    assert!(output.is_err());
    assert!(matches!(output.unwrap_err(), Error::EmptyString { .. }));
}

#[test]
fn should_fail_on_invalid_keywords() {
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
    let input = ":(top";

    assert!(!is_valid_in_git(input), "This pathspec is valid in git: {}", input);

    let output = git_pathspec::parse(input.as_bytes());
    assert!(output.is_err());
    assert!(matches!(output.unwrap_err(), Error::MissingClosingParenthesis { .. }));
}

fn check_valid_inputs(inputs: Vec<(&str, Pattern)>) {
    inputs.into_iter().for_each(|(input, expected)| {
        assert!(is_valid_in_git(input), "This pathspec is invalid in git: {}", input);

        let pattern = git_pathspec::parse(input.as_bytes()).expect("parsing should not fail");
        assert_eq!(pattern, expected, "while checking input: \"{}\"", input);
    });
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
