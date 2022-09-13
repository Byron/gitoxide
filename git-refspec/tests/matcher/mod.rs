use crate::matching;
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
    let mut out = Vec::new();
    matcher.match_remotes(matching::baseline::input(), &mut out);
    let _expected = matching::baseline::single(spec);
}
