#[cfg(test)]
mod acquire {
    use git_lock::acquire::Fail;

    #[test]
    fn fail_mode_immediately_produces_a_descriptive_error() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let resource = dir.path().join("the-resource");
        let _guard = git_lock::Marker::acquire_to_hold_resource(&resource, Fail::Immediately, None);
        let err_str = git_lock::Marker::acquire_to_hold_resource(resource, Fail::Immediately, None)
            .expect_err("the lock is taken and there is a failure obtaining it again")
            .to_string();

        assert!(err_str.contains("the-resource could not be obtained immediately"),);
        assert!(err_str.contains("the-resource.lock"),);
        Ok(())
    }
}
