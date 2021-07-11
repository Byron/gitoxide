use crate::file::transaction::prepare_and_commit::{committer, empty_store, reflog_lines};
use bstr::ByteSlice;
use git_actor::{Sign, Time};
use git_hash::ObjectId;
use git_lock::acquire::Fail;
use git_ref::{
    file::{log, WriteReflog},
    mutable::Target,
    transaction::{Change, Create, LogChange, RefEdit, RefLog},
};
use git_testtools::hex_to_id;
use std::{convert::TryInto, path::Path};

mod reference_with_equally_named {
    use crate::file::transaction::prepare_and_commit::{committer, empty_store};
    use git_lock::acquire::Fail;
    use git_ref::file::transaction;
    use git_ref::{
        file::WriteReflog,
        mutable::Target,
        transaction::{Change, Create, LogChange, RefEdit, RefLog},
    };
    use std::convert::TryInto;

    #[test]
    fn empty_or_non_empty_directory_already_in_place() -> crate::Result {
        for is_empty in &[true, false] {
            let (dir, store) = empty_store(WriteReflog::Normal)?;
            let head_dir = dir.path().join("HEAD");
            std::fs::create_dir_all(head_dir.join("a").join("b").join("also-empty"))?;
            if !*is_empty {
                std::fs::write(head_dir.join("file.ext"), "".as_bytes())?;
            }

            let edits = store
                .transaction(
                    Some(RefEdit {
                        change: Change::Update {
                            log: LogChange {
                                mode: RefLog::AndReference,
                                force_create_reflog: false,
                                message: Default::default(),
                            },
                            mode: Create::Only,
                            new: Target::Symbolic("refs/heads/main".try_into().unwrap()),
                        },
                        name: "HEAD".try_into()?,
                        deref: false,
                    }),
                    Fail::Immediately,
                )
                .commit(&committer());
            if *is_empty {
                let edits = edits?;
                assert!(
                    store.find_one(edits[0].name.to_partial())?.is_some(),
                    "HEAD was created despite a directory being in the way"
                );
            } else {
                let err = edits.unwrap_err();
                match err {
                    transaction::Error::LockCommit { err, full_name } => {
                        assert_eq!(full_name, "HEAD");
                        #[cfg(not(target_os = "windows"))]
                        assert_eq!(err.to_string(), "Directory not empty");
                    }
                    _ => unreachable!("other errors shouldn't happen here"),
                };
            }
        }
        Ok(())
    }
}

#[test]
#[ignore]
fn reference_with_old_value_must_exist_when_creating_it_and_have_that_value() {}

#[test]
#[ignore]
fn reference_without_old_value_must_not_exist_already_when_creating_it() {}

#[test]
fn symbolic_head_missing_referent_then_update_referent() -> crate::Result {
    for reflog_writemode in &[WriteReflog::Normal, WriteReflog::Disable] {
        let (_keep, store) = empty_store(*reflog_writemode)?;
        let referent = "refs/heads/alt-main";
        assert!(store.find_one(referent)?.is_none(), "the reference does not exist");
        let log_ignored = LogChange {
            mode: RefLog::AndReference,
            force_create_reflog: false,
            message: "ignored".into(),
        };
        let new_head_value = Target::Symbolic(referent.try_into().unwrap());
        let edits = store
            .transaction(
                Some(RefEdit {
                    change: Change::Update {
                        log: log_ignored.clone(),
                        new: new_head_value.clone(),
                        mode: Create::Only, // TODO: check failure if it doesn't exist
                    },
                    name: "HEAD".try_into()?,
                    deref: false,
                }),
                Fail::Immediately,
            )
            .commit(&committer())?;
        assert_eq!(
            edits,
            vec![RefEdit {
                change: Change::Update {
                    log: log_ignored.clone(),
                    new: new_head_value.clone(),
                    mode: Create::Only,
                },
                name: "HEAD".try_into()?,
                deref: false,
            }],
            "no split was performed"
        );

        let head = store.find_one_existing(edits[0].name.to_partial())?;
        assert_eq!(head.relative_path(), Path::new("HEAD"));
        assert_eq!(head.kind(), git_ref::Kind::Symbolic);
        assert_eq!(head.target().as_name(), Some(referent.as_bytes().as_bstr()));
        assert!(!head.log_exists()?, "no reflog is written for symbolic ref");
        assert!(store.find_one(referent)?.is_none(), "referent wasn't created");

        let new_oid = hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242");
        let new = Target::Peeled(new_oid);
        let log = LogChange {
            message: "an actual change".into(),
            mode: RefLog::AndReference,
            force_create_reflog: false,
        };
        let log_only = {
            let mut l = log.clone();
            l.mode = RefLog::Only;
            l
        };
        let edits = store
            .transaction(
                Some(RefEdit {
                    change: Change::Update {
                        log: log.clone(),
                        new: new.clone(),
                        mode: Create::OrUpdate { previous: None },
                    },
                    name: "HEAD".try_into()?,
                    deref: true,
                }),
                Fail::Immediately,
            )
            .commit(&committer())?;

        assert_eq!(
            edits,
            vec![
                RefEdit {
                    change: Change::Update {
                        log: log_only.clone(),
                        new: new.clone(),
                        mode: Create::OrUpdate {
                            previous: Some(new_head_value.clone())
                        },
                    },
                    name: "HEAD".try_into()?,
                    deref: false,
                },
                RefEdit {
                    change: Change::Update {
                        log,
                        new: new.clone(),
                        mode: Create::Only,
                    },
                    name: referent.try_into()?,
                    deref: false,
                }
            ]
        );

        let head = store.find_one_existing("HEAD")?;
        assert_eq!(
            head.kind(),
            git_ref::Kind::Symbolic,
            "head is still symbolic, not detached"
        );
        assert_eq!(
            head.target().as_name(),
            Some(referent.as_bytes().as_bstr()),
            "it still points to the referent"
        );

        let referent_ref = store.find_one_existing(referent)?;
        assert_eq!(referent_ref.kind(), git_ref::Kind::Peeled, "referent is a peeled ref");
        assert_eq!(
            referent_ref.target().as_id(),
            Some(new_oid.as_ref()),
            "referent points to desired hash"
        );

        let mut buf = Vec::new();
        for ref_name in &["HEAD", referent] {
            match reflog_writemode {
                WriteReflog::Normal => {
                    let expected_line = log::mutable::Line {
                        previous_oid: ObjectId::null_sha1(),
                        new_oid,
                        signature: git_actor::Signature {
                            name: "committer".into(),
                            email: "committer@example.com".into(),
                            time: Time {
                                time: 1234,
                                offset: 1800,
                                sign: Sign::Plus,
                            },
                        },
                        message: "an actual change".into(),
                    };
                    assert_eq!(reflog_lines(&store, *ref_name, &mut buf)?, vec![expected_line.clone()]);
                }
                WriteReflog::Disable => {
                    assert!(
                        store.reflog_iter(*ref_name, &mut buf)?.is_none(),
                        "nothing is ever written if its disabled"
                    )
                }
            }
        }
    }
    Ok(())
}

#[test]
#[should_panic]
fn referent_that_head_is_pointing_to() {
    // for reflog_writemode in &[git_ref::file::WriteReflog::Normal, git_ref::file::WriteReflog::Disable] {}
    todo!("verify that HEAD gets a reflog update automatically")
}

mod cancel_after_preparation {}
