use crate::matching;
use git_refspec::parse::Operation;
use git_refspec::MatchGroup;

#[test]
fn fetch_only() {
    let spec = git_refspec::parse("refs/heads/main".into(), Operation::Fetch).unwrap();
    let match_group = MatchGroup::from_fetch_specs(Some(spec));

    let actual = match_group.match_remotes(matching::baseline::input());

    let expected = matching::baseline::single(spec).unwrap();
    assert_eq!(
        actual.len(),
        expected.len(),
        "got a different amount of mappings: {:?} != {:?}",
        actual,
        expected
    );
    for (idx, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(actual.lhs, &expected.remote, "{}: remote mismatch", idx);
        if let Some(expected) = expected.local.as_ref() {
            match actual.rhs.as_ref() {
                None => panic!("{}: Expected local ref to be {}, got none", idx, expected),
                Some(actual) => assert_eq!(actual.as_ref(), expected, "{}: mismatched local ref", idx),
            }
        }
    }
}
