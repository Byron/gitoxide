mod prepare {
    mod create {
        use bstr::ByteSlice;
        use git_ref::{file, mutable, mutable::Target};
        use std::{convert::TryInto, path::Path};

        #[test]
        #[should_panic]
        fn reference_with_equally_named_empty_directory_already_in_place() {
            todo!("lock file renaming of a.lock to a but a is an empty directory")
        }

        #[test]
        #[should_panic]
        fn reference_with_equally_named_non_empty_directory_already_in_place() {
            todo!("lock file renaming of a.lock to a but a is a non-empty directory")
        }

        #[test]
        #[should_panic]
        fn reference_without_old_value_must_not_exist_already_when_creating_it() {
            todo!("lock file renaming of a.lock to a but a is a non-empty directory")
        }

        #[test]
        #[should_panic]
        fn symbolic_missing_referent() {
            for reflog_writemode in &[git_ref::file::WriteReflog::Normal, git_ref::file::WriteReflog::Disable] {
                let dir = tempfile::TempDir::new().unwrap();
                let mut store: file::Store = dir.path().to_owned().into();
                store.write_reflog = *reflog_writemode;
                let referent = "refs/heads/alt-main";
                assert!(
                    store.find_one(referent).unwrap().is_none(),
                    "the reference does not exist"
                );
                let t = store.transaction(
                    Some(mutable::RefEdit {
                        edit: mutable::Change::Update(mutable::Update {
                            mode: mutable::Reflog::AutoAndNoDeref,
                            new: Target::Symbolic(referent.try_into().unwrap()),
                            previous: None, // TODO: check failure if it doesn't exist
                        }),
                        name: "HEAD".try_into().unwrap(),
                    }),
                    git_lock::acquire::Fail::Immediately,
                );
                let edits = t.commit().unwrap();
                assert_eq!(edits.len(), 1, "no split was performed");
                let written = store.find_one_existing(edits[0].name.to_partial()).unwrap();
                assert_eq!(written.relative_path, Path::new("HEAD"));
                assert_eq!(written.kind(), git_ref::Kind::Symbolic);
                assert_eq!(written.target().as_name(), Some(referent.as_bytes().as_bstr()));
                assert!(!written.log_exists().unwrap(), "no revlog is written for symbolic ref");
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
