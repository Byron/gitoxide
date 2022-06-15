mod from_bytes {
    use git_ref::bstr::{BString, ByteSlice};
    use git_repository as git;
    use git_repository::RevSpec;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    const FIXTURE_NAME: &str = "make_rev_spec_parse_repos.sh";
    static BASELINE: Lazy<HashMap<BString, u8>> = Lazy::new(|| {
        let mut m = HashMap::new();
        let base = git_testtools::scripted_fixture_repo_read_only(FIXTURE_NAME).unwrap();
        let baseline = std::fs::read(base.join("baseline.git")).unwrap();
        let mut lines = baseline.lines();
        while let Some(spec) = lines.next() {
            let exit_code: u8 = lines.next().expect("exit code").to_str().unwrap().parse().unwrap();
            assert_eq!(
                m.insert(spec.into(), exit_code),
                None,
                "Duplicate spec '{}' cannot be handled",
                spec.as_bstr()
            );
        }
        m
    });

    fn parse_spec<'a>(spec: &str, repo: &'a git::Repository) -> Result<RevSpec<'a>, git::rev_spec::parse::Error> {
        let res = RevSpec::from_bstr(spec, repo);
        let expected = res.is_ok().then(|| 0).unwrap_or(128);
        let spec: BString = spec.into();
        assert_eq!(
            BASELINE.get(&spec),
            Some(&expected),
            "git baseline boiled down to success or failure must match our outcome"
        );
        res
    }

    fn repo(name: &str) -> crate::Result<git::Repository> {
        let base = git_testtools::scripted_fixture_repo_read_only(FIXTURE_NAME)?;
        Ok(git::open(base.join(name))?)
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
