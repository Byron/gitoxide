use gix_revision::spec;

use crate::spec::parse::{parse, try_parse};

#[test]
fn at_by_iteself_is_shortcut_for_head() {
    let rec = parse("@");
    assert!(rec.kind.is_none());
    assert_eq!(rec.get_ref(0), "HEAD");
}

#[test]
fn multiple_ats_are_invalid_but_may_cause_callbacks() {
    let err = try_parse("@@").unwrap_err();
    assert!(matches!(err, spec::parse::Error::AtNeedsCurlyBrackets {input} if input == "@"));
}

#[test]
fn lonely_at_after_ref_is_invalid() {
    let err = try_parse("HEAD@").unwrap_err();
    assert!(matches!(err, spec::parse::Error::AtNeedsCurlyBrackets {input} if input == "@"));
}

#[test]
fn refname_head() {
    let rec = parse("HEAD");
    assert!(rec.kind.is_none());
    assert_eq!(rec.get_ref(0), "HEAD");
}

#[test]
fn refname_tag() {
    let spec = "v1.2.3.4-beta.1";
    let rec = parse(spec);
    assert!(rec.kind.is_none());
    assert_eq!(rec.get_ref(0), spec);
}

#[test]
fn refname_with_head_prefix() {
    let rec = parse("HEADfake");
    assert!(rec.kind.is_none());
    assert_eq!(rec.get_ref(0), "HEADfake");
}

#[test]
fn full_head_ref_name() {
    let rec = parse("refs/heads/main");
    assert!(rec.kind.is_none());
    assert_eq!(rec.get_ref(0), "refs/heads/main");
}
