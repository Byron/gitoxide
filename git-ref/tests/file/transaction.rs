mod prepare {
    mod create {
        use crate::file::store_writable;
        use git_hash::ObjectId;
        use git_ref::edit;
        use git_ref::mutable::Target;

        #[test]
        #[should_panic]
        fn peeled() {
            for reflog_writemode in &[git_ref::file::WriteReflog::Normal, git_ref::file::WriteReflog::Disable] {
                let (_keep_dir, store) = store_writable().unwrap();
                let t = store.transaction(Some(edit::Reference {
                    edit: edit::Change::Update(edit::Update::new(
                        edit::Reflog::AutoAndNoDeref,
                        Target::Peeled(ObjectId::from_hex(b"12345678901234567890").unwrap()),
                        None,
                    )),
                    name: "refs/heads/newly-created".into(),
                }));
            }
        }

        mod cancel {}
    }
}
