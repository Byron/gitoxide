mod existing {
    use std::path::{Component, PathBuf};

    use git_repository::Kind;

    #[test]
    fn from_bare_git_dir() -> crate::Result {
        let dir = repo_path()?.join("bare.git");
        let path = git_repository::path::discover::existing(&dir)?;
        assert_eq!(path.as_ref(), dir, "the bare .git dir is directly returned");
        assert_eq!(path.kind(), Kind::Bare);
        Ok(())
    }

    #[test]
    fn from_inside_bare_git_dir() -> crate::Result {
        let git_dir = repo_path()?.join("bare.git");
        let dir = git_dir.join("objects");
        let path = git_repository::path::discover::existing(&dir)?;
        assert_eq!(
            path.as_ref(),
            git_dir,
            "the bare .git dir is found while traversing upwards"
        );
        assert_eq!(path.kind(), Kind::Bare);
        Ok(())
    }

    #[test]
    fn from_git_dir() -> crate::Result {
        let dir = repo_path()?.join(".git");
        let path = git_repository::path::discover::existing(&dir)?;
        assert_eq!(path.kind(), Kind::WorkingTree);
        assert_eq!(
            path.into_repository_directory(),
            dir,
            "the .git dir is directly returned if valid"
        );
        Ok(())
    }

    #[test]
    fn from_working_dir() -> crate::Result {
        let dir = repo_path()?;
        let path = git_repository::path::discover::existing(&dir)?;
        assert_eq!(path.as_ref(), dir, "a working tree dir yields the git dir");
        assert_eq!(path.kind(), Kind::WorkingTree);
        Ok(())
    }

    #[test]
    fn from_nested_dir() -> crate::Result {
        let working_dir = repo_path()?;
        let dir = working_dir.join("some/very/deeply/nested/subdir");
        let path = git_repository::path::discover::existing(&dir)?;
        assert_eq!(path.kind(), Kind::WorkingTree);
        assert_eq!(path.as_ref(), working_dir, "a working tree dir yields the git dir");
        Ok(())
    }

    #[test]
    fn from_dir_with_dot_dot() -> crate::Result {
        // This would be neater if we could just change the actual working directory,
        // but Rust tests run in parallel by default so we'd interfere with other tests.
        // Instead ensure it finds the gitoxide repo instead of a test repo if we crawl
        // up far enough. (This tests that `discover::existing` canonicalizes paths before
        // exploring ancestors.)
        let working_dir = repo_path()?;
        let dir = working_dir.join("some/very/deeply/nested/subdir/../../../../../../..");
        let path = git_repository::path::discover::existing(&dir)?;
        assert_eq!(path.kind(), Kind::WorkingTree);
        assert_eq!(
            path.as_ref()
                .components()
                .filter(|c| matches!(c, Component::ParentDir | Component::CurDir))
                .count(),
            0,
            "there are no relative path components anymore"
        );
        assert_ne!(
            path.as_ref().canonicalize()?,
            working_dir.canonicalize()?,
            "a relative path that climbs above the test repo should yield the gitoxide repo"
        );
        Ok(())
    }

    #[test]
    fn from_nested_dir_inside_a_git_dir() -> crate::Result {
        let working_dir = repo_path()?;
        let dir = working_dir.join(".git").join("objects");
        let path = git_repository::path::discover::existing(&dir)?;
        assert_eq!(path.kind(), Kind::WorkingTree);
        assert_eq!(path.as_ref(), working_dir, "we find .git directories on the way");
        Ok(())
    }

    fn repo_path() -> crate::Result<PathBuf> {
        git_testtools::scripted_fixture_repo_read_only("make_basic_repo.sh")
    }
}
