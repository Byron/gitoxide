mod from_bytes {
    use git_repository as git;
    use git_repository::RevSpec;

    fn parse_spec<'a>(spec: &str, repo: &'a git::Repository) -> Result<RevSpec<'a>, git::rev_spec::parse::Error> {
        RevSpec::from_bstr(spec, &repo)
    }

    fn repo(name: &str) -> crate::Result<git::Repository> {
        let repo_path = git_testtools::scripted_fixture_repo_read_only("make_rev_spec_parse_repos.sh")?;
        Ok(git::open(repo_path.join(name))?)
    }

    mod ambiguous {
        use super::repo;
        use crate::rev_spec::from_bytes::parse_spec;

        #[test]
        fn prefix() {
            {
                let repo = repo("blob.prefix").unwrap();
                assert_eq!(
                    parse_spec("dead", &repo).unwrap_err().to_string(),
                    "Found more than one object prefixed with dead\nThe ref partially named 'dead' could not be found"
                );
                assert_eq!(
                    parse_spec("beef", &repo).unwrap_err().to_string(),
                    "Found more than one object prefixed with beef\nThe ref partially named 'beef' could not be found"
                );
            }

            {
                let repo = repo("blob.bad").unwrap();
                assert_eq!(
                    parse_spec("bad0", &repo).unwrap_err().to_string(),
                    "Found more than one object prefixed with bad0\nThe ref partially named 'bad0' could not be found",
                    "git is able to also detect that the object has an invalid type, but we are not because the type doesn't matter here"
                );
            }
        }
    }

    #[test]
    #[ignore]
    fn bad_objects_are_valid_as_they_are_not_queried() {
        let repo = repo("blob.bad").unwrap();
        assert_eq!(
            parse_spec("e328", &repo).unwrap_err().to_string(),
            "we are able to return objects even though they are 'bad' when trying to decode them, like git",
        );
    }

    #[test]
    #[ignore]
    fn find_ref() {}
}
