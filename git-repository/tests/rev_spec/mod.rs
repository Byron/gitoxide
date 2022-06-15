mod from_bytes {
    mod ambiguous {
        use git_repository as git;
        use git_repository::RevSpec;

        fn repo(name: &str) -> crate::Result<git::Repository> {
            let repo_path = git_testtools::scripted_fixture_repo_read_only("make_rev_parse_disambiguation_repo.sh")?;
            Ok(git::open(repo_path.join(name))?)
        }

        #[test]
        fn prefix() {
            let repo = repo("blob.prefix").unwrap();
            assert_eq!(
                RevSpec::from_bstr("dead", &repo).unwrap_err().to_string(),
                "Found more than one object prefixed with dead\nThe ref partially named 'dead' could not be found"
            );
            assert_eq!(
                RevSpec::from_bstr("beef", &repo).unwrap_err().to_string(),
                "Found more than one object prefixed with beef\nThe ref partially named 'beef' could not be found"
            );
        }
    }
    #[test]
    #[ignore]
    fn find_ref() {}
}
