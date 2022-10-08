mod prefix {
    use git_refspec::{parse::Operation, RefSpec};

    #[test]
    fn partial_refs_have_no_prefix() {
        assert_eq!(parse("main").to_ref().prefix(), None);
    }

    #[test]
    fn short_absolute_refs_have_no_prefix() {
        assert_eq!(parse("refs/short").to_ref().prefix(), None);
    }

    #[test]
    fn full_names_have_a_prefix() {
        assert_eq!(parse("refs/heads/main").to_ref().prefix().unwrap(), "refs/heads/");
        assert_eq!(parse("refs/foo/bar").to_ref().prefix().unwrap(), "refs/foo/");
        assert_eq!(
            parse("refs/heads/*:refs/remotes/origin/*").to_ref().prefix().unwrap(),
            "refs/heads/"
        );
    }

    #[test]
    fn strange_glob_patterns_have_no_prefix() {
        assert_eq!(parse("refs/*/main:refs/*/main").to_ref().prefix(), None);
    }

    #[test]
    fn object_names_have_no_prefix() {
        assert_eq!(
            parse("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391").to_ref().prefix(),
            None
        );
    }

    fn parse(spec: &str) -> RefSpec {
        git_refspec::parse(spec.into(), Operation::Fetch).unwrap().to_owned()
    }
}
