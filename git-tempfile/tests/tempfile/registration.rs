mod mark_path {
    use git_tempfile::{AutoRemove, ContainingDirectory};

    #[test]
    fn it_persists_markers_along_with_newly_created_directories() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let target = dir.path().join("a").join("b").join("file.tmp");
        let new_filename = target.parent().unwrap().join("file.ext");
        drop(
            git_tempfile::mark_path(
                &target,
                ContainingDirectory::CreateAllRaceProof(Default::default()),
                AutoRemove::TempfileAndEmptyParentDirectoriesUntil {
                    boundary_directory: dir.path().into(),
                },
            )?
            .take()
            .expect("still there")
            .persist(&new_filename)?,
        );
        assert!(!target.exists(), "tempfile was renamed");
        assert!(
            new_filename.is_file(),
            "new file was placed (and parent directories still exist)"
        );
        Ok(())
    }

    #[test]
    fn it_can_create_the_containing_directory_and_remove_it_on_drop() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let first_dir = "dir";
        let filename = dir.path().join(first_dir).join("subdir").join("file.tmp");
        let tempfile = git_tempfile::mark_path(
            &filename,
            ContainingDirectory::CreateAllRaceProof(Default::default()),
            AutoRemove::TempfileAndEmptyParentDirectoriesUntil {
                boundary_directory: dir.path().into(),
            },
        )?;
        assert!(filename.is_file(), "specified file should exist precisely");
        drop(tempfile);
        assert!(
            !filename.is_file(),
            "after drop named files are deleted as well as extra directories"
        );
        assert!(
            !dir.path().join(first_dir).is_dir(),
            "previously created and now empty directories are deleted, too"
        );
        Ok(())
    }
}
mod at_path {
    use git_tempfile::{AutoRemove, ContainingDirectory};
    use std::io::Write;

    #[test]
    fn it_persists_tempfiles_along_with_newly_created_directories() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let target = dir.path().join("a").join("b").join("file.tmp");
        let new_filename = target.parent().unwrap().join("file.ext");
        let mut file = git_tempfile::at_path(
            &target,
            ContainingDirectory::CreateAllRaceProof(Default::default()),
            AutoRemove::TempfileAndEmptyParentDirectoriesUntil {
                boundary_directory: dir.path().into(),
            },
        )?
        .take()
        .expect("still there");
        file.write_all(b"hello world")?;
        drop(file.persist(&new_filename)?);
        assert!(!target.exists(), "tempfile was renamed");
        assert!(
            new_filename.is_file(),
            "new file was placed (and parent directories still exist)"
        );
        assert_eq!(
            std::fs::read(new_filename)?,
            &b"hello world"[..],
            "written content is persisted, too"
        );
        Ok(())
    }

    #[test]
    fn it_can_create_the_containing_directory_and_remove_it_on_drop() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let first_dir = "dir";
        let filename = dir.path().join(first_dir).join("subdir").join("file.tmp");
        let tempfile = git_tempfile::at_path(
            &filename,
            ContainingDirectory::CreateAllRaceProof(Default::default()),
            AutoRemove::TempfileAndEmptyParentDirectoriesUntil {
                boundary_directory: dir.path().into(),
            },
        )?;
        assert!(filename.is_file(), "specified file should exist precisely");
        drop(tempfile);
        assert!(
            !filename.is_file(),
            "after drop named files are deleted as well as extra directories"
        );
        assert!(
            !dir.path().join(first_dir).is_dir(),
            "previously created and now empty directories are deleted, too"
        );
        assert!(dir.path().is_dir(), "it won't touch the containing directory");
        Ok(())
    }

    #[test]
    fn it_names_files_correctly_and_removes_them_when_out_of_scope() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("something-specific.ext");
        let tempfile = git_tempfile::at_path(&filename, ContainingDirectory::Exists, AutoRemove::Tempfile)?;
        assert!(filename.is_file(), "specified file should exist precisely");
        drop(tempfile);
        assert!(!filename.is_file(), "after drop named files are deleted as well");
        assert!(dir.path().is_dir(), "it won't touch the containing directory");
        Ok(())
    }
}

mod new {
    use git_tempfile::{AutoRemove, ContainingDirectory};
    use std::path::Path;

    fn filecount_in(path: impl AsRef<Path>) -> usize {
        std::fs::read_dir(path).expect("valid dir").count()
    }

    #[test]
    fn it_can_be_kept() -> crate::Result {
        let dir = tempfile::tempdir()?;
        drop(
            git_tempfile::new(dir.path(), ContainingDirectory::Exists, AutoRemove::Tempfile)?
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
            let _keep = git_tempfile::new(dir.path(), ContainingDirectory::Exists, AutoRemove::Tempfile);
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
                AutoRemove::TempfileAndEmptyParentDirectoriesUntil {
                    boundary_directory: dir.path().into(),
                },
            );
            assert_eq!(
                filecount_in(&dir),
                1,
                "a temp file was created, as well as the directory"
            );
        }
        assert!(!containing_dir.is_dir(), "the now empty directory was deleted as well");
        assert!(dir.path().is_dir(), "it won't touch the containing directory");
        Ok(())
    }
}
