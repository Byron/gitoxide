mod find_remote {
    use crate::remote;
    use git_repository as git;
    use git_repository::remote::Direction;
    use git_repository::Repository;

    #[test]
    fn typical() {
        let repo = remote::repo("clone");
        let mut count = 0;
        let base_dir = base_dir(&repo);
        let expected = [
            (".", "+refs/heads/*:refs/remotes/myself/*"),
            (base_dir.as_str(), "+refs/heads/*:refs/remotes/origin/*"),
        ];
        for (name, (url, refspec)) in repo.remote_names().into_iter().zip(expected) {
            count += 1;
            let remote = repo.find_remote(name).expect("no error");
            assert_eq!(remote.name(), Some(name));

            let url = git::url::parse(url.as_bytes()).expect("valid");
            assert_eq!(remote.url(Direction::Fetch), Some(&url));

            let refspec = git::refspec::parse(refspec.into(), git::refspec::parse::Operation::Fetch)
                .expect("valid expectation")
                .to_owned();
            assert_eq!(
                remote.refspecs(Direction::Fetch),
                &[refspec],
                "default refspecs are set by git"
            );
            assert_eq!(
                remote.refspecs(Direction::Push),
                &[],
                "push-specs aren't configured by default"
            );
        }
        assert!(count > 0, "should have seen more than one commit");
        assert!(matches!(
            repo.find_remote("unknown").unwrap_err(),
            git::remote::find::existing::Error::NotFound { .. }
        ));
    }

    #[test]
    fn push_url_and_push_specs() {
        let repo = remote::repo("push-url");
        let remote = repo.find_remote("origin").expect("present");
        assert_eq!(remote.url(Direction::Push).unwrap().path, ".");
        assert_eq!(remote.url(Direction::Fetch).unwrap().path, base_dir(&repo));
        let spec = git::refspec::parse("refs/tags/*:refs/tags/*".into(), git::refspec::parse::Operation::Push)
            .unwrap()
            .to_owned();
        assert_eq!(remote.refspecs(Direction::Push), &[spec])
    }

    fn base_dir(repo: &Repository) -> String {
        repo.work_dir()
            .unwrap()
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .join("base")
            .display()
            .to_string()
    }
}
