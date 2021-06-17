mod iter {
    mod create_dir {
        use git_tempfile::iter;

        #[test]
        fn an_existing_directory_causes_immediate_success() -> crate::Result {
            let dir = tempfile::tempdir()?;
            let mut it = iter::CreateDir::new(dir.path());
            assert!(
                matches!(it.next(), Some(Ok(d)) if d == dir.path()),
                "first iteration is immediately successful"
            );
            assert!(it.next().is_none(), "iterator exhausted afterwards");
            Ok(())
        }

        #[test]
        fn a_single_directory_can_be_created_too() -> crate::Result {
            let dir = tempfile::tempdir()?;
            let new_dir = dir.path().join("new");
            let mut it = iter::CreateDir::new(&new_dir);
            assert!(
                matches!(it.next(), Some(Ok(d)) if d == new_dir),
                "first iteration is immediately successful"
            );
            assert!(it.next().is_none(), "iterator exhausted afterwards");
            assert!(new_dir.is_dir(), "the directory exists");
            Ok(())
        }

        // #[test]
        // fn multiple_intermediate_directories_are_created_automaticaly() -> crate::Result {
        //     let dir = tempfile::tempdir()?;
        //     let new_dir = dir.path().join("s1").join("s2").join("new");
        //     let mut it = iter::CreateDir::new(&new_dir);
        //     assert!(
        //         matches!(it.next(), Some(Err(e)) if d == new_dir),
        //         "first iteration is immediately successful"
        //     );
        //     assert!(it.next().is_none(), "iterator exhausted afterwards");
        //     assert!(new_dir.is_dir(), "the directory exists");
        //     Ok(())
        // }
    }
}

mod registration;
mod force_setup {
    #[test]
    fn can_be_called_multiple_times() {
        // we could probably be smart and figure out that this does the right thing, but… it's good enough it won't fail ;).
        git_tempfile::force_setup(git_tempfile::SignalHandlerMode::DeleteTempfilesOnTermination);
        git_tempfile::force_setup(
            git_tempfile::SignalHandlerMode::DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour,
        );
    }
}
