use crate::spec::parse::try_parse;
use git_revision::spec;

#[test]
fn braces_must_be_closed() {
    for unclosed_spec in ["@{something", "@{", "@{..@"] {
        let err = try_parse(unclosed_spec).unwrap_err();
        assert!(matches!(err, spec::parse::Error::UnclosedBracePair {input} if input == unclosed_spec[1..]))
    }
}

mod at {}
