use gix_attributes::State;
use gix_pathspec::{MagicSignature, MatchMode};

use crate::parse::{check_valid_inputs, NormalizedPattern};

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

fn pat_with_path(path: &str) -> NormalizedPattern {
    pat_with_path_and_sig(path, MagicSignature::empty())
}

fn pat_with_path_and_sig(path: &str, signature: MagicSignature) -> NormalizedPattern {
    pat(path, signature, MatchMode::ShellGlob, vec![])
}

fn pat_with_sig(signature: MagicSignature) -> NormalizedPattern {
    pat("", signature, MatchMode::ShellGlob, vec![])
}

fn pat_with_attrs(attrs: Vec<(&'static str, State)>) -> NormalizedPattern {
    pat("", MagicSignature::empty(), MatchMode::ShellGlob, attrs)
}

fn pat_with_search_mode(search_mode: MatchMode) -> NormalizedPattern {
    pat("", MagicSignature::empty(), search_mode, vec![])
}

fn pat(
    path: &str,
    signature: MagicSignature,
    search_mode: MatchMode,
    attributes: Vec<(&str, State)>,
) -> NormalizedPattern {
    NormalizedPattern {
        path: path.into(),
        signature,
        search_mode,
        attributes: attributes
            .into_iter()
            .map(|(attr, state)| (attr.into(), state))
            .collect(),
    }
}
