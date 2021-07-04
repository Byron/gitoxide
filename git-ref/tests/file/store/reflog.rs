fn store() -> crate::Result<git_ref::file::Store> {
    Ok(git_ref::file::Store::at(
        git_testtools::scripted_fixture_repo_read_only("make_repo_for_reflog.sh")?.join(".git"),
        git_ref::file::WriteReflog::Disable,
    ))
}

mod iter {
    use crate::file::store::reflog::store;

    #[test]
    fn non_existing_returns_none() -> crate::Result {
        let store = store()?;
        let mut buf = Vec::new();
        assert!(
            matches!(store.reflog_iter("FAILURE", &mut buf), Ok(None)),
            "this one does not exist"
        );
        Ok(())
    }

    #[test]
    fn for_head_and_main() -> crate::Result {
        let store = store()?;
        let mut buf = Vec::new();

        let log = store.reflog_iter("HEAD", &mut buf)?.expect("exists");
        assert_eq!(log.filter_map(Result::ok).count(), 5);

        let log = store.reflog_iter("refs/heads/main", &mut buf)?.expect("exists");
        assert_eq!(log.filter_map(Result::ok).count(), 5);
        Ok(())
    }
}
