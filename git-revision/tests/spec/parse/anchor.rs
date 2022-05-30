use crate::spec::parse::{parse, try_parse, try_parse_opts, Options};
use git_revision::spec;

#[test]
fn full_describe_ouptput_parses_hash_portion_as_prefix() {
    let rec = parse("cargo-smart-release-679-g3bee7fb");
    assert!(rec.kind.is_none());
    assert_eq!(
        rec.resolve_ref_input, None,
        "references are not resolved in describe output"
    );
    assert_eq!(rec.prefix, Some(git_hash::Prefix::from_hex("3bee7fb").unwrap()));
    assert_eq!(rec.calls, 1);
}

#[test]
fn full_describe_ouptput_lookalikes_are_still_treated_as_ref() {
    let spec = "cargo-smart-release-679-g3bee7fb";
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
    assert_eq!(rec.calls, 2, "call prefix, then call ref");
}

#[test]
fn any_hash_without_suffix_and_prefix_g_is_assumed_to_be_describe_output() {
    let spec = "foo--bar-gabcdef1";
    let rec = parse(spec);
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input, None);
    assert_eq!(
        rec.prefix,
        Some(git_hash::Prefix::from_hex("abcdef1").unwrap()),
        "git does not parse very precisely here"
    );
    assert_eq!(rec.calls, 1);

    for invalid_describe in ["-gabcdef1", "gabcdef1"] {
        let rec = parse(invalid_describe);
        assert!(rec.kind.is_none());
        assert_eq!(
            rec.resolve_ref_input.unwrap(),
            invalid_describe,
            "we don't consider this a prefix from a describe block"
        );
        assert_eq!(rec.prefix, None);
        assert_eq!(rec.calls, 1);
    }
}

#[test]
fn full_describe_ouptput_with_dirty_suffix_is_recognized() {
    let rec = parse("cargo-smart-release-679-g3bee7fb-dirty");
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input, None, "git does not see this as prefix, we do");
    assert_eq!(rec.prefix, Some(git_hash::Prefix::from_hex("3bee7fb").unwrap()),);
    assert_eq!(rec.calls, 1);
}

#[test]
fn partial_describe_ouptput_with_dirty_suffix_is_recognized() {
    let spec = "abcdef1-dirty";
    let rec = parse(spec);
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input, None,);
    assert_eq!(
        rec.prefix,
        Some(git_hash::Prefix::from_hex("abcdef1").unwrap()),
        "git does not see this as prefix anymore, we do"
    );
    assert_eq!(rec.calls, 1);
}

#[test]
fn partial_describe_ouptput_lookalikes_are_never_considered() {
    let spec = "abcdef1-dirty-laundry";
    let rec = parse(spec);
    assert!(rec.kind.is_none());
    assert_eq!(rec.resolve_ref_input.unwrap(), spec);
    assert_eq!(rec.prefix, None,);
    assert_eq!(rec.calls, 1, "we don't even try the prefix");
}

#[test]
fn partial_describe_ouptput_with_dirty_suffix_lookalikes_are_treated_as_refs() {
    let spec = "abcdef1-dirty";
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
    assert_eq!(rec.prefix, None,);
    assert_eq!(rec.calls, 2);
}

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
