mod prepare_and_commit {
    use git_ref::file;

    fn empty_store(log_mode: git_ref::file::WriteReflog) -> crate::Result<file::Store> {
        let dir = tempfile::TempDir::new().unwrap();
        let mut store: file::Store = dir.path().to_owned().into();
        store.write_reflog = log_mode;
        Ok(store)
    }

    mod create {
        use crate::file::transaction::prepare_and_commit::empty_store;
        use bstr::ByteSlice;
        use git_lock::acquire::Fail;
        use git_ref::transaction::{Change, RefEdit, Reflog, Target};
        use std::{convert::TryInto, path::Path};

        mod reference_with_equally_named {
            #[test]
            #[ignore]
            fn empty_directory_already_in_place() {
                todo!("lock file renaming of a.lock to a but a is an empty directory")
            }

            #[test]
            #[ignore]
            fn non_empty_directory_already_in_place() {
                todo!("lock file renaming of a.lock to a but a is a non-empty directory")
            }
        }

        #[test]
        #[ignore]
        fn reference_without_old_value_must_not_exist_already_when_creating_it() {
            todo!("lock file renaming of a.lock to a but a is a non-empty directory")
        }

        #[test]
        fn symbolic_head_missing_referent() -> crate::Result {
            for reflog_writemode in &[git_ref::file::WriteReflog::Normal, git_ref::file::WriteReflog::Disable] {
                let store = empty_store(*reflog_writemode)?;
                let referent = "refs/heads/alt-main";
                assert!(store.find_one(referent)?.is_none(), "the reference does not exist");
                let t = store.transaction(
                    Some(RefEdit {
                        change: Change::Update {
                            mode: Reflog::AutoAndNoDeref,
                            new: Target::Symbolic(referent.try_into()?),
                            previous: None, // TODO: check failure if it doesn't exist
                        },
                        name: "HEAD".try_into()?,
                    }),
                    Fail::Immediately,
                );
                let edits = t.commit()?;
                assert_eq!(edits.len(), 1, "no split was performed");
                let written = store.find_one_existing(edits[0].name.to_partial())?;
                assert_eq!(written.relative_path(), Path::new("HEAD"));
                assert_eq!(written.kind(), git_ref::Kind::Symbolic);
                assert_eq!(written.target().as_name(), Some(referent.as_bytes().as_bstr()));
                assert!(!written.log_exists()?, "no revlog is written for symbolic ref");
            }
            Ok(())
        }

        #[test]
        #[ignore]
        fn referent_that_head_is_pointing_to() {
            todo!("verify that HEAD gets a reflog update automatically")
        }

        mod cancel_after_preparation {}
    }

    mod update {
        #[test]
        #[ignore]
        fn write_head_and_reference_transparently() {
            todo!("writing a head being a symbolic ref writes through to the referent in an extra refedit")
        }
    }

    mod delete {
        use crate::file::store_writable;
        use crate::file::transaction::prepare_and_commit::empty_store;
        use git_hash::ObjectId;
        use git_lock::acquire::Fail;
        use git_ref::{
            file::WriteReflog,
            transaction::{Change, RefEdit, Target},
        };
        use std::convert::TryInto;

        #[test]
        fn delete_a_ref_which_is_gone_succeeds() {
            let store = empty_store(WriteReflog::Normal).unwrap();
            let edits = store
                .transaction(
                    Some(RefEdit {
                        change: Change::Delete { previous: None },
                        name: "DOES_NOT_EXIST".try_into().unwrap(),
                    }),
                    Fail::Immediately,
                )
                .commit()
                .unwrap();
            assert_eq!(edits.len(), 1);
        }

        #[test]
        fn delete_a_ref_which_is_gone_but_must_exist_fails() {
            let store = empty_store(WriteReflog::Normal).unwrap();
            let res = store
                .transaction(
                    Some(RefEdit {
                        change: Change::Delete {
                            previous: Some(Target::Peeled(ObjectId::null_sha1())),
                        },
                        name: "DOES_NOT_EXIST".try_into().unwrap(),
                    }),
                    Fail::Immediately,
                )
                .commit();
            match res {
                Ok(_) => unreachable!("must exist, but it doesn't actually exist"),
                Err(err) => assert_eq!(
                    err.to_string(),
                    "The reference 'DOES_NOT_EXIST' for deletion did not exist"
                ),
            }
        }

        #[test]
        #[should_panic]
        fn delete_reflog_only_of_symbolic_no_deref() {
            let (_keep, store) = store_writable("make_repo_for_reflog.sh").unwrap();
            let head = store.find_one_existing("HEAD").unwrap();
            assert!(head.log_exists().unwrap());

            let edits = store
                .transaction(
                    Some(RefEdit {
                        change: Change::Delete {
                            previous: Some(Target::Peeled(ObjectId::null_sha1())),
                        },
                        name: head.name().into(),
                    }),
                    Fail::Immediately,
                )
                .commit()
                .unwrap();

            assert_eq!(edits.len(), 1);
            let head = store.find_one_existing("HEAD").unwrap();
            assert!(!head.log_exists().unwrap());
        }

        #[test]
        #[ignore]
        fn delete_reflog_only_of_symbolic_with_deref() {
            let _store = store_writable("make_repo_for_reflog.sh").unwrap();
            todo!("assure it won't delete the ref")
        }

        #[test]
        #[ignore]
        fn delete_broken_ref_that_must_exist() {
            todo!("this should definitely work")
        }
    }
}
