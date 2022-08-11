mod remote_at {
    use crate::remote;
    use git_repository::remote::Direction;

    #[test]
    fn url_and_push_url() -> crate::Result {
        let repo = remote::repo("base");
        let fetch_url = "https://github.com/byron/gitoxide";
        let remote = repo.remote_at(fetch_url)?;

        assert_eq!(remote.name(), None);
        assert_eq!(remote.url(Direction::Fetch).unwrap().to_bstring()?, fetch_url);
        assert_eq!(remote.url(Direction::Push).unwrap().to_bstring()?, fetch_url);

        let remote = remote.push_url("user@host.xz:./relative")?;
        assert_eq!(
            remote.url(Direction::Push).unwrap().to_bstring()?,
            "ssh://user@host.xz/relative"
        );
        assert_eq!(remote.url(Direction::Fetch).unwrap().to_bstring()?, fetch_url);

        Ok(())
    }

    #[test]
    fn url_rewrites_are_respected() -> crate::Result {
        let repo = remote::repo("url-rewriting");
        let remote = repo.remote_at("https://github.com/foobar/gitoxide")?;

        assert_eq!(remote.name(), None, "anonymous remotes are unnamed");
        let rewritten_fetch_url = "https://github.com/byron/gitoxide";
        assert_eq!(
            remote.url(Direction::Fetch).unwrap().to_bstring()?,
            rewritten_fetch_url,
            "fetch was rewritten"
        );
        assert_eq!(
            remote.url(Direction::Push).unwrap().to_bstring()?,
            rewritten_fetch_url,
            "push is the same as fetch was rewritten"
        );

        let remote = repo
            .remote_at("https://github.com/foobar/gitoxide".to_owned())?
            .push_url("file://dev/null".to_owned())?;
        assert_eq!(remote.url(Direction::Fetch).unwrap().to_bstring()?, rewritten_fetch_url);
        assert_eq!(
            remote.url(Direction::Push).unwrap().to_bstring()?,
            "ssh://dev/null",
            "push-url rewrite rules are applied"
        );
        Ok(())
    }
}

mod find_remote {
    use crate::remote;
    use git_object::bstr::BString;
    use git_repository as git;
    use git_repository::remote::Direction;
    use git_repository::Repository;
    use std::io::BufRead;

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
            assert_eq!(remote.url(Direction::Fetch).unwrap(), &url);

            assert_eq!(
                remote.refspecs(Direction::Fetch),
                &[fetchspec(refspec)],
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
        assert_eq!(remote.refspecs(Direction::Push), &[pushspec("refs/tags/*:refs/tags/*")])
    }

    #[test]
    fn many_fetchspecs() {
        let repo = remote::repo("many-fetchspecs");
        let remote = repo.find_remote("origin").expect("present");
        assert_eq!(
            remote.refspecs(Direction::Fetch),
            &[
                fetchspec("HEAD"),
                fetchspec("+refs/heads/*:refs/remotes/origin/*"),
                fetchspec("refs/tags/*:refs/tags/*")
            ]
        )
    }

    #[test]
    fn instead_of_url_rewriting() -> crate::Result {
        let repo = remote::repo("url-rewriting");

        let baseline = std::fs::read(repo.git_dir().join("baseline.git"))?;
        let mut baseline = baseline.lines().filter_map(Result::ok);
        let expected_fetch_url: BString = baseline.next().expect("fetch").into();
        let expected_push_url: BString = baseline.next().expect("push").into();

        let remote = repo.find_remote("origin")?;
        assert_eq!(remote.url(Direction::Fetch).unwrap().to_bstring()?, expected_fetch_url,);
        {
            let actual_push_url = remote.url(Direction::Push).unwrap().to_bstring()?;
            assert_ne!(
                actual_push_url, expected_push_url,
                "here we actually resolve something that git doesn't for unknown reason"
            );
            assert_eq!(
                actual_push_url, "ssh://dev/null",
                "file:// gets replaced actually and it's a valid url"
            );
        }

        let remote = remote.apply_url_aliases(false);
        assert_eq!(
            remote.url(Direction::Fetch).unwrap().to_bstring()?,
            "https://github.com/foobar/gitoxide"
        );
        assert_eq!(remote.url(Direction::Push).unwrap().to_bstring()?, "file://dev/null");
        Ok(())
    }

    fn fetchspec(spec: &str) -> git_refspec::RefSpec {
        git::refspec::parse(spec.into(), git::refspec::parse::Operation::Fetch)
            .unwrap()
            .to_owned()
    }

    fn pushspec(spec: &str) -> git_refspec::RefSpec {
        git::refspec::parse(spec.into(), git::refspec::parse::Operation::Push)
            .unwrap()
            .to_owned()
    }

    fn base_dir(repo: &Repository) -> String {
        git_path::to_unix_separators_on_windows(git::path::into_bstr(
            git::path::realpath(repo.work_dir().unwrap())
                .unwrap()
                .parent()
                .unwrap()
                .join("base"),
        ))
        .into_owned()
        .to_string()
    }
}
