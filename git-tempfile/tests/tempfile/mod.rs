mod iter {
    mod create_dir {
        use git_tempfile::iter;

        #[test]
        fn an_existing_directory_causes_immediate_success() -> crate::Result {
            let dir = tempfile::tempdir()?;
            let mut it = iter::CreateDir::new(dir.path());
            assert!(
                matches!(it.next(), Some(Ok(()))),
                "first iteration is immediately successful"
            );
            assert!(it.next().is_none(), "iterator exhausted aferwards");
            Ok(())
        }
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
