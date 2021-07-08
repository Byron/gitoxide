mod prepare_and_commit {
    use git_ref::file;

    fn empty_store(log_mode: git_ref::file::WriteReflog) -> crate::Result<(tempfile::TempDir, file::Store)> {
        let dir = tempfile::TempDir::new().unwrap();
        let mut store: file::Store = dir.path().to_owned().into();
        store.write_reflog = log_mode;
        Ok((dir, store))
    }

    mod create;

    mod update {
        #[test]
        #[ignore]
        fn write_head_via_reference_transparently() {
            todo!("writing a ref which happens to be (special case) referred to by HEAD alters HEADs reflog, too.")
        }
    }

    mod delete;
}
