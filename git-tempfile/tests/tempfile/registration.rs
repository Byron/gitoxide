mod at_path {
    use git_tempfile::ContainingDirectory;

    #[test]
    fn it_can_create_the_containing_directory_and_remove_it_on_drop() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let first_dir = "dir";
        let filename = dir.path().join(first_dir).join("subdir").join("file.tmp");
        let tempfile = git_tempfile::at_path(&filename, ContainingDirectory::CreateAllRaceProof(Default::default()))?;
        assert!(filename.is_file(), "specified file should exist precisely");
        drop(tempfile);
        assert!(
            !filename.is_file(),
            "after drop named files are deleted as well as extra directories"
        );
        // assert!(
        //     !dir.path().join(first_dir).is_dir(),
        //     "previously created and now empty directories are deleted, too"
        // ); // TODO
        Ok(())
    }

    #[test]
    fn it_names_files_correctly_and_removes_them_when_out_of_scope() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("something-specific.ext");
        let tempfile = git_tempfile::at_path(&filename, ContainingDirectory::Exists)?;
        assert!(filename.is_file(), "specified file should exist precisely");
        drop(tempfile);
        assert!(!filename.is_file(), "after drop named files are deleted as well");
        Ok(())
    }
}

mod new {
    use git_tempfile::ContainingDirectory;
    use std::path::Path;

    fn filecount_in(path: impl AsRef<Path>) -> usize {
        std::fs::read_dir(path).expect("valid dir").count()
    }

    #[test]
    fn it_can_be_kept() -> crate::Result {
        let dir = tempfile::tempdir()?;
        drop(
            git_tempfile::new(dir.path(), ContainingDirectory::Exists)?
                .take()
                .expect("not taken yet")
                .keep()?,
        );
        assert_eq!(filecount_in(&dir), 1, "a temp file and persisted");
        Ok(())
    }

    #[test]
    fn it_is_removed_if_it_goes_out_of_scope() -> crate::Result {
        let dir = tempfile::tempdir()?;
        {
            let _keep = git_tempfile::new(dir.path(), ContainingDirectory::Exists);
            assert_eq!(filecount_in(&dir), 1, "a temp file was created");
        }
        assert_eq!(filecount_in(&dir), 0, "lock was automatically removed");
        Ok(())
    }

    #[test]
    fn it_can_create_the_containing_directory_and_remove_it_when_dropped() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let containing_dir = dir.path().join("dir");
        assert!(!containing_dir.exists());
        {
            let _keep = git_tempfile::new(
                &containing_dir,
                ContainingDirectory::CreateAllRaceProof(Default::default()),
            );
            assert_eq!(
                filecount_in(&dir),
                1,
                "a temp file was created, as well as the directory"
            );
        }
        // TODO:
        // assert!(!containing_dir.is_dir(), "the now empty directory was deleted as well");
        Ok(())
    }
}
