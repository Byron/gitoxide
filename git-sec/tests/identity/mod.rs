mod uid {
    #[test]
    fn from_path() -> crate::Result {
        let dir = tempfile::tempdir()?;
        assert!(git_sec::identity::is_path_owned_by_current_user(dir.path().into())?);
        Ok(())
    }
}
