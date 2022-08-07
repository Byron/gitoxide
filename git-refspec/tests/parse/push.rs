use crate::parse::{assert_parse, b, try_parse};
use git_refspec::{parse::Error, Instruction, Mode, Operation, Push};

#[test]
fn negative_unsupported() {
    for spec in ["^a:b", "^a:", "^:", "^:b", "^"] {
        assert!(matches!(
            try_parse(spec, Operation::Push).unwrap_err(),
            Error::NegativeUnsupported
        ));
    }
}

#[test]
fn ampersand_is_resolved_to_head() {
    assert_parse(
        "@",
        Instruction::Push(Push::Matching {
            src: b("HEAD"),
            dst: b("HEAD"),
            allow_non_fast_forward: false,
        }),
    );

    assert_parse(
        "+@",
        Instruction::Push(Push::Matching {
            src: b("HEAD"),
            dst: b("HEAD"),
            allow_non_fast_forward: true,
        }),
    );
}

#[test]
fn lhs_colon_rhs_pushes_single_ref() {
    assert_parse(
        "a:b",
        Instruction::Push(Push::Matching {
            src: b("a"),
            dst: b("b"),
            allow_non_fast_forward: false,
        }),
    );
    assert_parse(
        "+a:b",
        Instruction::Push(Push::Matching {
            src: b("a"),
            dst: b("b"),
            allow_non_fast_forward: true,
        }),
    );
    assert_parse(
        "a/*:b/*",
        Instruction::Push(Push::Matching {
            src: b("a/*"),
            dst: b("b/*"),
            allow_non_fast_forward: false,
        }),
    );
    assert_parse(
        "+a/*:b/*",
        Instruction::Push(Push::Matching {
            src: b("a/*"),
            dst: b("b/*"),
            allow_non_fast_forward: true,
        }),
    );
}

#[test]
fn colon_alone_is_for_pushing_matching_refs() {
    assert_parse(
        ":",
        Instruction::Push(Push::AllMatchingBranches {
            allow_non_fast_forward: false,
        }),
    );
    assert_parse(
        "+:",
        Instruction::Push(Push::AllMatchingBranches {
            allow_non_fast_forward: true,
        }),
    );
}

#[test]
fn delete() {
    assert_parse(":a", Instruction::Push(Push::Delete { ref_or_pattern: b("a") }));
    let spec = assert_parse("+:a", Instruction::Push(Push::Delete { ref_or_pattern: b("a") }));
    assert_eq!(
        spec.mode(),
        Mode::Force,
        "force is set, even though it has no effect in the actual instruction"
    );
}
