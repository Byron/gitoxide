use crate::spec::parse::{parse, try_parse, try_parse_opts, Options};
use git_revision::spec;

#[test]
fn short_hash_likes_are_considered_prefixes() {
    let rec = parse("abCD");
    assert!(rec.kind.is_none());
    assert_eq!(
        rec.resolve_ref_input, None,
        "references are not resolved if prefix lookups succeed"
    );
    assert_eq!(rec.prefix, Some(git_hash::Prefix::from_hex("abcd").unwrap()));
    assert_eq!(rec.calls, 1);

    let rec = parse("gabcd123");
    assert!(rec.kind.is_none());
    assert_eq!(
        rec.resolve_ref_input.unwrap(),
        "gabcd123",
        "ref lookups are performed if it doesn't look like a hex sha"
    );
    assert_eq!(
        rec.prefix, None,
        "prefix lookups are not attempted at all (and they are impossible even)"
    );
    assert_eq!(rec.calls, 1);
}

#[test]
fn unresolvable_hash_likes_are_resolved_as_refs() {
    let rec = try_parse_opts(
        "abCD",
        Options {
            reject_prefix: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input.unwrap(), "abCD");
    assert_eq!(rec.prefix, None);
    assert_eq!(rec.calls, 2);
}

#[test]
fn hash_likes_that_are_too_long_are_resolved_as_refs() {
    let spec = "abcd123456789abcd123456789abcd123456789abcd123456789abcd123456789abcd123456789abcd123456789";
    let rec = try_parse_opts(
        spec,
        Options {
            reject_prefix: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input.unwrap(), spec);
    assert_eq!(rec.prefix, None);
    assert_eq!(
        rec.calls, 1,
        "we can't create a prefix from it, hence only ref resolution is attempted"
    );
}

#[test]
fn at_by_iteself_is_shortcut_for_head() {
    let rec = parse("@");
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
}

#[test]
fn multiple_ats_are_invalid_but_may_cause_callbacks() {
    let err = try_parse("@@").unwrap_err();
    assert!(matches!(err, spec::parse::Error::UnconsumedInput {input} if input == "@"));
}

#[test]
fn lonely_at_after_ref_is_invalid() {
    let err = try_parse("HEAD@").unwrap_err();
    assert!(matches!(err, spec::parse::Error::AtNeedsCurlyBrackets {input} if input == ""));
}

#[test]
fn refname_head() {
    let rec = parse("HEAD");
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
}

#[test]
fn refname_with_head_prefix() {
    let rec = parse("HEADfake");
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input.unwrap(), "HEADfake");
}

#[test]
fn full_head_ref_name() {
    let rec = parse("refs/heads/main");
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input.unwrap(), "refs/heads/main");
}
