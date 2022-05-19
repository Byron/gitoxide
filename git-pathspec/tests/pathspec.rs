use bstr::BString;
use git_pathspec::{MagicSignature, Pattern};

#[test]
fn can_parse() {
    let inputs = vec![
        ("some/path", pat("some/path", None)),
        ("some/*.path", pat("some/*.path", None)),
        (":/", pat("", Some(MagicSignature::TOP))),
        (":^", pat("", Some(MagicSignature::EXCLUDE))),
        (":!", pat("", Some(MagicSignature::EXCLUDE))),
        (":(top)", pat("", Some(MagicSignature::TOP))),
        (":(literal)", pat("", Some(MagicSignature::LITERAL))),
        (":(icase)", pat("", Some(MagicSignature::ICASE))),
        (":(glob)", pat("", Some(MagicSignature::GLOB))),
        (":(attr)", pat("", Some(MagicSignature::ATTR))),
        (":(exclude)", pat("", Some(MagicSignature::EXCLUDE))),
        (
            ":(top,literal,icase,glob,attr,exclude)some/path",
            pat(
                "some/path",
                Some(
                    MagicSignature::TOP
                        | MagicSignature::LITERAL
                        | MagicSignature::ICASE
                        | MagicSignature::GLOB
                        | MagicSignature::ATTR
                        | MagicSignature::EXCLUDE,
                ),
            ),
        ),
        (":/:some/path", pat("some/path", Some(MagicSignature::TOP))),
        (
            ":!(literal)some/*path",
            pat("some/*path", Some(MagicSignature::EXCLUDE | MagicSignature::LITERAL)),
        ),
        (":", pat("", None)),
        (":()", pat("", None)),
        (":::::", pat("", None)),
        (":!/!/:", pat("", Some(MagicSignature::TOP | MagicSignature::EXCLUDE))),
    ];

    for (input, expected) in inputs {
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
    ];

    for (input, expected) in inputs {
        let output = git_pathspec::parse(input.as_bytes());
        assert!(output.is_err());
        assert_eq!(output.unwrap_err(), expected);
    }
}

fn pat(path: &str, signature: Option<MagicSignature>) -> Pattern {
    Pattern {
        path: path.into(),
        signature,
    }
}
