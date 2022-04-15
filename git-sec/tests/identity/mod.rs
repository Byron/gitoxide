mod uid {
    #[test]
    fn from_path() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let file = dir.path().join("file");
        std::fs::write(&file, &[])?;
        assert!(git_sec::identity::is_path_owned_by_current_user(file.into())?);
        assert!(git_sec::identity::is_path_owned_by_current_user(dir.path().into())?);
        Ok(())
    }
}
