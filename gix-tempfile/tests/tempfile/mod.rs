mod fs;
mod handle;

mod setup {
    #[test]
    fn can_be_called_multiple_times() {
        // we could probably be smart and figure out that this does the right thing, but… it's good enough it won't fail ;).
        gix_tempfile::setup(gix_tempfile::SignalHandlerMode::DeleteTempfilesOnTermination);
        gix_tempfile::setup(gix_tempfile::SignalHandlerMode::DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour);
    }
}
