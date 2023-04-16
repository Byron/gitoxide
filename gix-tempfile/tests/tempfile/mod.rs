mod handle;

#[cfg(feature = "signals")]
mod setup {
    #[test]
    fn can_be_called_multiple_times() {
        // we could probably be smart and figure out that this does the right thing, but… it's good enough it won't fail ;).
        gix_tempfile::signal::setup(gix_tempfile::signal::handler::Mode::DeleteTempfilesOnTermination);
        gix_tempfile::signal::setup(
            gix_tempfile::signal::handler::Mode::DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour,
        );
    }
}
