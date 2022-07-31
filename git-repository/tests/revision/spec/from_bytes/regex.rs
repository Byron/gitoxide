use crate::revision::spec::from_bytes::{parse_spec_no_baseline, repo};
use git_repository::prelude::ObjectIdExt;
use git_testtools::hex_to_id;

mod with_known_revision {
    use super::*;
    use crate::revision::spec::from_bytes::parse_spec;
    use git_repository::revision::Spec;

    #[test]
    #[cfg(not(feature = "regex"))]
    fn contained_string_matches_in_unanchored_regex_and_disambiguates_automatically() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        let expected = Spec::from_id(hex_to_id("0000000000e4f9fbd19cf1e932319e5ad0d1d00b").attach(&repo));

        assert_eq!(parse_spec("0000000000^{/x}", &repo).unwrap(), expected);
        assert_eq!(parse_spec("@^{/x}", &repo).unwrap(), expected, "ref names are resolved");

        assert_eq!(
            parse_spec_no_baseline("@^{/.*x}", &repo).unwrap_err().to_string(),
            "None of 1 commits from 0000000000e matched text \".*x\"",
            "regexes are not actually available for us, but git could do that"
        );
    }

    #[test]
    #[cfg(feature = "regex")]
    fn contained_string_matches_in_unanchored_regex_and_disambiguates_automatically() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        let expected = Spec::from_id(hex_to_id("0000000000e4f9fbd19cf1e932319e5ad0d1d00b").attach(&repo));

        assert_eq!(
            parse_spec("0000000000^{/x}", &repo).unwrap(),
            expected,
            "search is unanchored by default"
        );
        assert_eq!(
            parse_spec("@^{/x}", &repo).unwrap(),
            expected,
            "ref names are resolved as well"
        );

        assert_eq!(
            parse_spec("@^{/^.*x}", &repo).unwrap(),
            expected,
            "we can use real regexes here"
        );
        assert_eq!(
            parse_spec_no_baseline("@^{/^x}", &repo).unwrap_err().to_string(),
            "None of 1 commits from 0000000000e matched regex \"^x\"",
        );
    }
}

mod find_youngest_matching_commit {
    use super::*;
    use crate::revision::spec::from_bytes::parse_spec;
    use git_repository::revision::Spec;

    #[test]
    #[cfg(not(feature = "regex"))]
    #[ignore]
    fn contained_string_matches() {
        let repo = repo("complex_graph").unwrap();

        assert_eq!(
            parse_spec(":/message", &repo).unwrap(),
            Spec::from_id(hex_to_id("ef80b4b77b167f326351c93284dc0eb00dd54ff4").attach(&repo))
        );
    }

    #[test]
    #[cfg(feature = "regex")]
    #[ignore]
    fn regex_matches() {
        let repo = repo("complex_graph").unwrap();

        assert_eq!(
            parse_spec(":/mes.age", &repo).unwrap(),
            Spec::from_id(hex_to_id("ef80b4b77b167f326351c93284dc0eb00dd54ff4").attach(&repo))
        );
    }
}
