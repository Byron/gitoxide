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
        use git_ref::{
            mutable::Target,
            transaction::{Change, RefEdit, UpdateMode},
        };
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
                            mode: UpdateMode::RefAndRefLog {
                                create_unconditionally: false,
                            },
                            new: Target::Symbolic(referent.try_into()?),
                            previous: None, // TODO: check failure if it doesn't exist
                            deref: false,
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
            mutable::Target,
            transaction::{Change, DeleteMode, RefEdit},
        };
        use std::convert::TryInto;

        #[test]
        fn delete_a_ref_which_is_gone_succeeds() {
            let store = empty_store(WriteReflog::Normal).unwrap();
            let edits = store
                .transaction(
                    Some(RefEdit {
                        change: Change::Delete {
                            previous: None,
                            mode: DeleteMode::RefAndRefLog,
                            deref: false,
                        },
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
                            mode: DeleteMode::RefAndRefLog,
                            deref: false,
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
        fn delete_ref_and_reflog_on_symbolic_no_deref() -> crate::Result {
            let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
            let head = store.find_one_existing("HEAD")?;
            assert!(head.log_exists().unwrap());
            let _main = store.find_one_existing("main")?;

            let edits = store
                .transaction(
                    Some(RefEdit {
                        change: Change::Delete {
                            previous: Some(Target::Peeled(ObjectId::null_sha1())),
                            mode: DeleteMode::RefAndRefLog,
                            deref: false,
                        },
                        name: head.name().into(),
                    }),
                    Fail::Immediately,
                )
                .commit()?;

            assert_eq!(
                edits,
                vec![RefEdit {
                    change: Change::Delete {
                        previous: Some(Target::Symbolic("refs/heads/main".try_into()?)),
                        mode: DeleteMode::RefAndRefLog,
                        deref: false
                    },
                    name: head.name().into(),
                }],
                "the previous value was updated with the actual one"
            );
            assert!(
                store.reflog_iter_rev("HEAD", &mut [0u8; 128])?.is_none(),
                "reflog was deleted"
            );
            assert!(store.find_one("HEAD")?.is_none(), "ref was deleted");
            assert!(store.find_one("main")?.is_some(), "referent still exists");
            Ok(())
        }

        #[test]
        fn delete_reflog_only_of_symbolic_no_deref() -> crate::Result {
            let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
            let head = store.find_one_existing("HEAD")?;
            assert!(head.log_exists().unwrap());

            let edits = store
                .transaction(
                    Some(RefEdit {
                        change: Change::Delete {
                            previous: Some(Target::Symbolic("refs/heads/main".try_into()?)),
                            mode: DeleteMode::RefLogOnly,
                            deref: false,
                        },
                        name: head.name().into(),
                    }),
                    Fail::Immediately,
                )
                .commit()?;

            assert_eq!(edits.len(), 1);
            let head = store.find_one_existing("HEAD")?;
            assert!(!head.log_exists().unwrap());
            let main = store.find_one_existing("main").expect("referent still exists");
            assert!(main.log_exists()?, "log is untouched, too");
            assert_eq!(
                main.target(),
                head.peel_one_level().expect("a symref")?.target(),
                "head points to main"
            );
            Ok(())
        }

        #[test]
        #[should_panic]
        fn delete_reflog_only_of_symbolic_with_deref() {
            let (_keep, store) = store_writable("make_repo_for_reflog.sh").unwrap();
            let head = store.find_one_existing("HEAD").unwrap();
            assert!(head.log_exists().unwrap());

            let edits = store
                .transaction(
                    Some(RefEdit {
                        change: Change::Delete {
                            previous: Some(Target::Symbolic("refs/heads/main".try_into().unwrap())),
                            mode: DeleteMode::RefLogOnly,
                            deref: true,
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
            let main = store.find_one_existing("main").expect("referent still exists");
            assert!(!main.log_exists().unwrap(), "log is removed");
            assert_eq!(
                main.target(),
                head.peel_one_level().expect("a symref").unwrap().target(),
                "head points to main"
            );
        }

        #[test]
        #[ignore]
        fn delete_broken_ref_that_must_exist() {
            todo!("this should definitely work")
        }
    }
}
