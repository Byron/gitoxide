mod from_git_dir_file {
    use git_testtools::tempfile;
    use git_testtools::tempfile::NamedTempFile;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    #[test]
    fn absolute_path() -> crate::Result {
        let (path, _) = write_and_read(b"gitdir: /absolute/path/.git")?;
        assert_eq!(path, Path::new("/absolute/path/.git"));
        Ok(())
    }

    #[test]
    fn relative_path_is_made_absolute_relative_to_containing_dir() -> crate::Result {
        let (path, gitdir_file) = write_and_read(b"gitdir: relative/path")?;
        assert_eq!(path, gitdir_file.parent().unwrap().join(Path::new("relative/path")));
        Ok(())
    }

    fn write_and_read(content: &[u8]) -> crate::Result<(PathBuf, PathBuf)> {
        let file = gitdir_with_content(content)?;
        Ok((git_discover::path::from_gitdir_file(file.path())?, file.path().into()))
    }

    fn gitdir_with_content(content: &[u8]) -> std::io::Result<NamedTempFile> {
        let mut file = tempfile::NamedTempFile::new()?;
        file.write_all(content)?;
        Ok(file)
    }
}
