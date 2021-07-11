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
fn reference_with_old_value_must_exist_when_creating_it_and_have_that_value() {}

#[test]
#[ignore]
fn reference_without_old_value_must_not_exist_already_when_creating_it() {}

#[test]
fn symbolic_head_missing_referent_then_update_referent() {
    for reflog_writemode in &[WriteReflog::Normal, WriteReflog::Disable] {
        let (_keep, store) = empty_store(*reflog_writemode).unwrap();
        let referent = "refs/heads/alt-main";
        assert!(
            store.find_one(referent).unwrap().is_none(),
            "the reference does not exist"
        );
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
                    name: "HEAD".try_into().unwrap(),
                    deref: false,
                }),
                Fail::Immediately,
            )
            .commit(&committer())
            .unwrap();
        assert_eq!(
            edits,
            vec![RefEdit {
                change: Change::Update {
                    log: log_ignored.clone(),
                    new: new_head_value.clone(),
                    mode: Create::Only,
                },
                name: "HEAD".try_into().unwrap(),
                deref: false,
            }],
            "no split was performed"
        );

        let head = store.find_one_existing(edits[0].name.to_partial()).unwrap();
        assert_eq!(head.relative_path(), Path::new("HEAD"));
        assert_eq!(head.kind(), git_ref::Kind::Symbolic);
        assert_eq!(head.target().as_name(), Some(referent.as_bytes().as_bstr()));
        assert!(!head.log_exists().unwrap(), "no reflog is written for symbolic ref");
        assert!(store.find_one(referent).unwrap().is_none(), "referent wasn't created");

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
                    name: "HEAD".try_into().unwrap(),
                    deref: true,
                }),
                Fail::Immediately,
            )
            .commit(&committer())
            .unwrap();

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
                    name: "HEAD".try_into().unwrap(),
                    deref: false,
                },
                RefEdit {
                    change: Change::Update {
                        log,
                        new: new.clone(),
                        mode: Create::Only,
                    },
                    name: referent.try_into().unwrap(),
                    deref: false,
                }
            ]
        );

        let head = store.find_one_existing("HEAD").unwrap();
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

        let referent_ref = store.find_one_existing(referent).unwrap();
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
                    assert_eq!(
                        reflog_lines(&store, *ref_name, &mut buf).unwrap(),
                        vec![expected_line.clone()]
                    );
                }
                WriteReflog::Disable => {
                    assert!(
                        store.reflog_iter(*ref_name, &mut buf).unwrap().is_none(),
                        "nothing is ever written if its disabled"
                    )
                }
            }
        }
    }
}

#[test]
#[should_panic]
fn referent_that_head_is_pointing_to() {
    // for reflog_writemode in &[git_ref::file::WriteReflog::Normal, git_ref::file::WriteReflog::Disable] {}
    todo!("verify that HEAD gets a reflog update automatically")
}

mod cancel_after_preparation {}
