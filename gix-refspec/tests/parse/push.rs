use gix_refspec::{
    instruction::Push,
    parse::{Error, Operation},
    Instruction,
};

use crate::parse::{assert_parse, b, try_parse};

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
fn revspecs_with_ref_name_destination() {
    assert_parse(
        "main~1:b",
        Instruction::Push(Push::Matching {
            src: b("main~1"),
            dst: b("b"),
            allow_non_fast_forward: false,
        }),
    );
    assert_parse(
        "+main~1:b",
        Instruction::Push(Push::Matching {
            src: b("main~1"),
            dst: b("b"),
            allow_non_fast_forward: true,
        }),
    );
}

#[test]
fn destinations_must_be_ref_names() {
    assert!(matches!(
        try_parse("a~1:b~1", Operation::Push).unwrap_err(),
        Error::ReferenceName(_)
    ));
}

#[test]
fn single_refs_must_be_refnames() {
    assert!(matches!(
        try_parse("a~1", Operation::Push).unwrap_err(),
        Error::ReferenceName(_)
    ));
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
    assert_parse("+:a", Instruction::Push(Push::Delete { ref_or_pattern: b("a") }));
}
