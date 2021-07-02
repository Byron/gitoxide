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
                let t = store.transaction(
                    Some(edit::RefEdit {
                        edit: edit::Change::Update(edit::Update {
                            mode: edit::Reflog::AutoAndNoDeref,
                            new: Target::Symbolic("refs/heads/alt-main".try_into().unwrap()),
                            previous: None, // TODO: check failure if it doesn't exist
                        }),
                        name: "NEW_HEAD".try_into().unwrap(),
                    }),
                    git_lock::acquire::Fail::Immediately,
                );
                let edits = t.commit().unwrap();
                assert_eq!(edits.len(), 1, "no split was performed");
                todo!("figure out a way to split")
            }
        }

        #[test]
        #[should_panic]
        fn referent_that_head_is_pointing_to() {
            todo!("verify that HEAD gets a reflog update automatically")
        }

        mod cancel_after_preparation {}
    }

    mod update {
        #[test]
        #[should_panic]
        fn write_head_and_reference_transparently() {
            todo!("writing a head being a symbolic ref writes through to the referent in an extra refedit")
        }
    }

    mod delete {
        #[test]
        #[should_panic]
        fn delete_a_ref_which_is_gone() {
            todo!("it's fine to do that")
        }
    }
}
