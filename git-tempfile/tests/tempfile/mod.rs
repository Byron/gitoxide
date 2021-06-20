mod fs;
mod handle;

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
