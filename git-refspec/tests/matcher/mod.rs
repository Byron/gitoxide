use crate::matching;
use git_refspec::matcher::Match;
use git_refspec::parse::Operation;

mod match_ {
    use git_refspec::matcher::Match;

    #[test]
    fn default_is_not_matched() {
        assert!(!Match::default().matched())
    }
}

#[test]
#[ignore]
fn fetch_only() {
    let spec = git_refspec::parse("refs/heads/main".into(), Operation::Fetch).unwrap();
    let matcher = spec.to_matcher();

    let mut actual = Vec::new();
    actual.extend(std::iter::repeat(Match::default()).take(matching::baseline::input().len()));
    matcher.match_remotes(matching::baseline::input().zip(actual.iter_mut()));
    actual.retain(|m| m.matched());

    let expected = matching::baseline::single(spec).unwrap();
    assert_eq!(
        actual.len(),
        expected.len(),
        "got a different amount of mappings: {:?} != {:?}",
        actual,
        expected
    );
    for (idx, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(
            actual.remote().expect("local matched"),
            &expected.remote,
            "{}: remote mismatch",
            idx
        );
        if let Some(expected) = expected.local.as_ref() {
            match actual.local() {
                None => panic!("{}: Expected local ref to be {}, got none", idx, expected),
                Some(actual) => assert_eq!(actual, expected, "{}: mismatched local ref", idx),
            }
        }
    }
}
