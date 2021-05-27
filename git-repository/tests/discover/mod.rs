mod existing {
    use std::path::PathBuf;

    #[test]
    fn from_bare_git_dir() -> crate::Result {
        let dir = repo_path()?.join("bare.git");
        assert_eq!(
            git_repository::discover::existing(&dir)?,
            dir,
            "the bare .git dir is directly returned"
        );
        Ok(())
    }

    #[test]
    fn from_inside_bare_git_dir() -> crate::Result {
        let git_dir = repo_path()?.join("bare.git");
        let dir = git_dir.join("objects");
        assert_eq!(
            git_repository::discover::existing(&dir)?,
            git_dir,
            "the bare .git dir is found while traversing upwards"
        );
        Ok(())
    }

    #[test]
    fn from_git_dir() -> crate::Result {
        let dir = repo_path()?.join(".git");
        assert_eq!(
            git_repository::discover::existing(&dir)?,
            dir,
            "the .git dir is directly returned if valid"
        );
        Ok(())
    }

    #[test]
    fn from_working_dir() -> crate::Result {
        let dir = repo_path()?;
        assert_eq!(
            git_repository::discover::existing(&dir)?,
            dir.join(".git"),
            "a working tree dir yields the git dir"
        );
        Ok(())
    }

    #[test]
    fn from_nested_dir() -> crate::Result {
        let working_dir = repo_path()?;
        let dir = working_dir.join("some/very/deeply/nested/subdir");
        assert_eq!(
            git_repository::discover::existing(&dir)?,
            working_dir.join(".git"),
            "a working tree dir yields the git dir"
        );
        Ok(())
    }

    #[test]
    fn from_nested_dir_inside_a_git_dir() -> crate::Result {
        let working_dir = repo_path()?;
        let dir = working_dir.join(".git").join("objects");
        assert_eq!(
            git_repository::discover::existing(&dir)?,
            working_dir.join(".git"),
            "we find .git directories on the way"
        );
        Ok(())
    }

    fn repo_path() -> crate::Result<PathBuf> {
        git_testtools::scripted_fixture_repo_read_only("make_basic_repo.sh")
    }
}
