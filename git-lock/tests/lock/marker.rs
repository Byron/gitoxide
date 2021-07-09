#[cfg(test)]
mod acquire {
    use git_lock::acquire::Fail;
    use std::time::{Duration, Instant};

    #[test]
    fn fail_mode_immediately_produces_a_descriptive_error() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let resource = dir.path().join("the-resource");
        let guard = git_lock::Marker::acquire_to_hold_resource(&resource, Fail::Immediately, None)?;
        assert!(guard.lock_path().ends_with("the-resource.lock"));
        let err_str = git_lock::Marker::acquire_to_hold_resource(resource, Fail::Immediately, None)
            .expect_err("the lock is taken and there is a failure obtaining it again")
            .to_string();

        assert!(err_str.contains("the-resource could not be obtained immediately"));
        assert!(err_str.contains("the-resource.lock"), "it mentions the lockfile itself");
        Ok(())
    }

    #[test]
    fn fail_mode_after_duration_fails_after_a_given_duration_or_more() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let resource = dir.path().join("the-resource");
        let _guard = git_lock::Marker::acquire_to_hold_resource(&resource, Fail::Immediately, None)?;
        let start = Instant::now();
        let time_to_wait = Duration::from_millis(50);
        let err_str =
            git_lock::Marker::acquire_to_hold_resource(resource, Fail::AfterDurationWithBackoff(time_to_wait), None)
                .expect_err("the lock is taken and there is a failure obtaining it again after some delay")
                .to_string();
        assert!(
            start.elapsed() >= time_to_wait,
            "it should never wait less than the given wait time"
        );
        assert!(
            err_str.contains("could not be obtained after 0.05s"),
            "it lets us know that we were waiting for some time"
        );
        assert!(err_str.contains("the-resource.lock"), "it mentions the lockfile itself");
        Ok(())
    }
}
mod commit {
    use git_lock::acquire::Fail;

    #[test]
    fn fails_for_ordinary_marker_that_was_never_writable() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let resource = dir.path().join("the-resource");
        let mark = git_lock::Marker::acquire_to_hold_resource(&resource, Fail::Immediately, None)?;
        let err = mark.commit().expect_err("should always fail");
        assert_eq!(err.kind(), std::io::ErrorKind::Other);
        assert_eq!(
            err.get_ref().expect("custom error").to_string(),
            "refusing to commit marker that was never opened"
        );
        Ok(())
    }
}
