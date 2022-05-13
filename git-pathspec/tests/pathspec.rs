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
        let pattern = git_pathspec::parse(input.as_bytes());
        assert_eq!(pattern, expected, "while checking input: \"{}\"", input);
    }
}

fn pat(path: &str, signature: Option<MagicSignature>) -> Pattern {
    Pattern {
        path: path.into(),
        signature,
    }
}

#[test]
#[ignore]
fn can_match() {
    // let buf = b"git-pathspec/tests/pathspec.rs";
    // git_pathspec::matches(buf);
}
