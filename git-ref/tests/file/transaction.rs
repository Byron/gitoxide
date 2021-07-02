mod prepare {
    mod create {
        use git_ref::{edit, file, mutable::Target};
        use std::convert::TryInto;

        #[test]
        #[should_panic]
        fn symbolic_missing_referent() {
            for reflog_writemode in &[git_ref::file::WriteReflog::Normal, git_ref::file::WriteReflog::Disable] {
                let dir = tempfile::TempDir::new().unwrap();
                let mut store: file::Store = dir.path().to_owned().into();
                store.write_reflog = *reflog_writemode;
                let _t = store.transaction(Some(edit::Reference {
                    edit: edit::Change::Update(edit::Update::new(
                        edit::Reflog::AutoAndNoDeref,
                        Target::Symbolic("refs/heads/alt-main".try_into().unwrap()),
                        None, // TODO: check failure if it doesn't exist
                    )),
                    name: "NEW_HEAD".try_into().unwrap(),
                }));
                todo!("figure out a way to split")
            }
        }

        mod cancel {}
    }
}
