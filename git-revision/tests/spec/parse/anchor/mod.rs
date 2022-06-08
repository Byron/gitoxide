use crate::spec::parse::try_parse;
use git_revision::spec;

mod at_symbol;
mod describe;
mod hash;
mod refnames;
mod colon_symbol {
    #[test]
    #[ignore]
    fn empty_top_level_regex_are_invalid() {
        // git also can't do it, finds nothing instead. It could be the youngest commit in theory, but isn't.
    }
}

#[test]
fn braces_must_be_closed() {
    for unclosed_spec in ["@{something", "@{", "@{..@"] {
        let err = try_parse(unclosed_spec).unwrap_err();
        assert!(matches!(err, spec::parse::Error::UnclosedBracePair {input} if input == unclosed_spec[1..]))
    }
}
