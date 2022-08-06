use crate::parse::{assert_parse, b};
use git_refspec::{Fetch, Instruction, Mode};

#[test]
fn exclude() {
    assert_parse("^a", Instruction::Fetch(Fetch::Exclude { src: b("a") }));
    assert_parse("^a*", Instruction::Fetch(Fetch::Exclude { src: b("a*") }));
}

#[test]
fn lhs_colon_empty_fetches_only() {
    assert_parse("src:", Instruction::Fetch(Fetch::Only { src: b("src") }));
    let spec = assert_parse("+src:", Instruction::Fetch(Fetch::Only { src: b("src") }));
    assert_eq!(
        spec.mode(),
        Mode::Force,
        "force is set, even though it has no effect in the actual instruction"
    );
}

#[test]
fn lhs_colon_rhs_updates_single_ref() {
    assert_parse(
        "a:b",
        Instruction::Fetch(Fetch::AndUpdate {
            src: b("a"),
            dst: b("b"),
            allow_non_fast_forward: false,
        }),
    );
    assert_parse(
        "+a:b",
        Instruction::Fetch(Fetch::AndUpdate {
            src: b("a"),
            dst: b("b"),
            allow_non_fast_forward: true,
        }),
    );

    assert_parse(
        "a/*:b/*",
        Instruction::Fetch(Fetch::AndUpdate {
            src: b("a/*"),
            dst: b("b/*"),
            allow_non_fast_forward: false,
        }),
    );
    assert_parse(
        "+a/*:b/*",
        Instruction::Fetch(Fetch::AndUpdate {
            src: b("a/*"),
            dst: b("b/*"),
            allow_non_fast_forward: true,
        }),
    );
}

#[test]
fn empty_lhs_colon_rhs_fetches_head_to_destination() {
    assert_parse(
        ":a",
        Instruction::Fetch(Fetch::AndUpdate {
            src: b("HEAD"),
            dst: b("a"),
            allow_non_fast_forward: false,
        }),
    );

    assert_parse(
        "+:a",
        Instruction::Fetch(Fetch::AndUpdate {
            src: b("HEAD"),
            dst: b("a"),
            allow_non_fast_forward: true,
        }),
    );
}

#[test]
fn colon_alone_is_for_fetching_head_into_fetchhead() {
    assert_parse(":", Instruction::Fetch(Fetch::Only { src: b("HEAD") }));
    let spec = assert_parse("+:", Instruction::Fetch(Fetch::Only { src: b("HEAD") }));
    assert_eq!(spec.mode(), Mode::Force, "it's set even though it's not useful");
}

#[test]
fn empty_refspec_is_enough_for_fetching_head_into_fetchhead() {
    assert_parse("", Instruction::Fetch(Fetch::Only { src: b("HEAD") }));
}
