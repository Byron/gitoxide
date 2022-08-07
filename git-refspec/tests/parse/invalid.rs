use crate::parse::try_parse;
use git_refspec::{parse::Error, Operation};

#[test]
fn empty() {
    assert!(matches!(try_parse("", Operation::Push).unwrap_err(), Error::Empty));
}

#[test]
fn negative_must_not_be_empty() {
    for op in [Operation::Fetch, Operation::Push] {
        assert!(matches!(try_parse("^", op).unwrap_err(), Error::NegativeEmpty));
    }
}

#[test]
fn negative_with_destination() {
    for op in [Operation::Fetch, Operation::Push] {
        for spec in ["^a:b", "^a:", "^:", "^:b"] {
            assert!(matches!(
                try_parse(spec, op).unwrap_err(),
                Error::NegativeWithDestination
            ));
        }
    }
}

#[test]
fn complex_patterns_with_more_than_one_asterisk() {
    for op in [Operation::Fetch, Operation::Push] {
        for spec in ["^*/*", "a/*/c/*", "a**:**b", "+:**/"] {
            assert!(matches!(
                try_parse(spec, op).unwrap_err(),
                Error::PatternUnsupported { .. }
            ));
        }
    }
}

#[test]
fn both_sides_need_pattern_if_one_uses_it() {
    for op in [Operation::Fetch, Operation::Push] {
        for spec in ["refs/*/a", ":a/*", "+:a/*", "a*:b/c", "a:b/*"] {
            assert!(
                matches!(try_parse(spec, op).unwrap_err(), Error::PatternUnbalanced),
                "{}",
                spec
            );
        }
    }
}

#[test]
fn push_to_empty() {
    assert!(matches!(
        try_parse("HEAD:", Operation::Push).unwrap_err(),
        Error::PushToEmpty
    ));
}
