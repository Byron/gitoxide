mod bare {
    use git_testtools::tempfile;

    #[test]
    fn init_into_non_existing_directory_creates_it() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        let git_dir = tmp.path().join("bare.git");
        let repo = git_repository::init_bare(&git_dir)?;
        assert_eq!(repo.kind(), git_repository::Kind::Bare);
        assert!(
            repo.work_dir().is_none(),
            "a worktree isn't present in bare repositories"
        );
        assert_eq!(
            repo.git_dir(),
            git_dir,
            "the repository is placed into the given directory without added sub-directories"
        );
        assert_eq!(git_repository::open(repo.git_dir())?, repo);
        Ok(())
    }

    #[test]
    fn init_into_empty_directory_uses_it_directly() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        let repo = git_repository::init_bare(tmp.path())?;
        assert_eq!(repo.kind(), git_repository::Kind::Bare);
        assert!(
            repo.work_dir().is_none(),
            "a worktree isn't present in bare repositories"
        );
        assert_eq!(
            repo.git_dir(),
            tmp.path(),
            "the repository is placed into the directory itself"
        );
        assert_eq!(git_repository::open(repo.git_dir())?, repo);
        Ok(())
    }

    #[test]
    fn init_into_non_empty_directory_is_not_allowed() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        std::fs::write(tmp.path().join("existing.txt"), b"I was here before you")?;

        assert!(git_repository::init_bare(tmp.path())
            .unwrap_err()
            .to_string()
            .starts_with("Refusing to initialize the non-empty directory as"));
        Ok(())
    }
}

mod non_bare {
    use git_repository as git;
    use git_testtools::tempfile;

    #[test]
    fn init_bare_with_custom_branch_name() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        let repo: git::Repository = git::ThreadSafeRepository::init_opts(
            tmp.path(),
            git::create::Kind::Bare,
            git::create::Options::default(),
            git::open::Options::isolated().config_overrides(Some("init.defaultBranch=special")),
        )?
        .into();
        assert_eq!(
            repo.head()?.referent_name().expect("name").as_bstr(),
            "refs/heads/special"
        );
        Ok(())
    }
    #[test]
    fn init_into_empty_directory_creates_a_dot_git_dir() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        let repo = git::init(tmp.path())?;
        assert_eq!(repo.kind(), git::Kind::WorkTree { is_linked: false });
        assert_eq!(repo.work_dir(), Some(tmp.path()), "there is a work tree by default");
        assert_eq!(
            repo.git_dir(),
            tmp.path().join(".git"),
            "there is a work tree by default"
        );
        assert_eq!(git::open(repo.git_dir())?, repo);
        assert_eq!(git::open(repo.work_dir().as_ref().expect("non-bare repo"))?, repo);
        Ok(())
    }

    #[test]
    fn init_into_non_empty_directory_is_not_allowed_if_option_is_set_as_used_for_clone() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        std::fs::write(tmp.path().join("existing.txt"), b"I was here before you")?;

        let err = git::ThreadSafeRepository::init_opts(
            tmp.path(),
            git::create::Kind::WithWorktree,
            git::create::Options {
                destination_must_be_empty: true,
                ..Default::default()
            },
            git::open::Options::isolated(),
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .starts_with("Refusing to initialize the non-empty directory as"));
        Ok(())
    }

    #[test]
    fn init_into_non_empty_directory_is_allowed_by_default() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        std::fs::write(tmp.path().join("existing.txt"), b"I was here before you")?;

        let repo = git_repository::init(tmp.path())?;
        assert_eq!(repo.work_dir().expect("present"), tmp.path());
        assert_eq!(
            repo.git_dir(),
            tmp.path().join(".git"),
            "gitdir is inside of the workdir"
        );
        Ok(())
    }
}
