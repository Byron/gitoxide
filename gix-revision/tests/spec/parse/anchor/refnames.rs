use gix_revision::spec;

use crate::spec::parse::{parse, try_parse};

#[test]
fn at_by_itself_is_shortcut_for_head() {
    let rec = parse("@");
    assert!(rec.kind.is_none());
    assert_eq!(rec.get_ref(0), "HEAD");
}

#[test]
fn at_is_allowed() {
    let rec = parse("a@b");
    assert!(rec.kind.is_none());
    assert_eq!(rec.get_ref(0), "a@b");
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
