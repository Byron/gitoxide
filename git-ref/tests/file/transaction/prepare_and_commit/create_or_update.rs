use crate::file::{
    store_writable,
    transaction::prepare_and_commit::{committer, empty_store, log_line, reflog_lines},
};
use bstr::ByteSlice;
use git_hash::ObjectId;
use git_lock::acquire::Fail;
use git_ref::{
    file::{transaction, WriteReflog},
    mutable::Target,
    transaction::{Change, Create, LogChange, RefEdit, RefLog},
};
use git_testtools::hex_to_id;
use std::convert::TryInto;

mod reference_with_equally_named {
    use crate::file::transaction::prepare_and_commit::{committer, empty_store};
    use git_lock::acquire::Fail;
    use git_ref::{
        file::transaction,
        mutable::Target,
        transaction::{Change, Create, LogChange, RefEdit},
    };
    use std::convert::TryInto;

    #[test]
    fn empty_or_non_empty_directory_already_in_place() -> crate::Result {
        for is_empty in &[true, false] {
            let (dir, store) = empty_store()?;
            let head_dir = dir.path().join("HEAD");
            std::fs::create_dir_all(head_dir.join("a").join("b").join("also-empty"))?;
            if !*is_empty {
                std::fs::write(head_dir.join("file.ext"), "".as_bytes())?;
            }

            let edits = store
                .transaction()
                .prepare(
                    Some(RefEdit {
                        change: Change::Update {
                            log: LogChange::default(),
                            mode: Create::Only,
                            new: Target::Symbolic("refs/heads/main".try_into().unwrap()),
                        },
                        name: "HEAD".try_into()?,
                        deref: false,
                    }),
                    Fail::Immediately,
                )?
                .commit(&committer());
            if *is_empty {
                let edits = edits?.0;
                assert!(
                    store.loose_find(edits[0].name.to_partial())?.is_some(),
                    "HEAD was created despite a directory being in the way"
                );
            } else {
                match edits {
                    #[cfg_attr(target_os = "windows", allow(unused_variables))]
                    Err(transaction::Error::LockCommit { err, full_name }) => {
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
fn reference_with_old_value_must_exist_when_creating_it() -> crate::Result {
    let (_keep, store) = empty_store()?;

    let res = store.transaction().prepare(
        Some(
            RefEdit {
                change: Change::Update {
                    log: LogChange::default(),
                    new: Target::Peeled(ObjectId::null_sha1()),
                    mode: Create::OrUpdate {
                        previous: Some(Target::must_exist()),
                    },
                },
                name: "HEAD".try_into()?,
                deref: false,
            }
            .clone(),
        ),
        Fail::Immediately,
    );

    match res {
        Err(transaction::Error::MustExist { full_name, expected }) => {
            assert_eq!(full_name, "HEAD");
            assert_eq!(expected, Target::must_exist());
        }
        _ => unreachable!("unexpected result"),
    }
    Ok(())
}

#[test]
fn reference_with_explicit_value_must_match_the_value_on_update() -> crate::Result {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
    let head = store.loose_find("HEAD")?.expect("head exists already");
    let target = head.target;

    let res = store.transaction().prepare(
        Some(
            RefEdit {
                change: Change::Update {
                    log: LogChange::default(),
                    new: Target::Peeled(ObjectId::null_sha1()),
                    mode: Create::OrUpdate {
                        previous: Some(Target::Peeled(hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"))),
                    },
                },
                name: "HEAD".try_into()?,
                deref: false,
            }
            .clone(),
        ),
        Fail::Immediately,
    );
    match res {
        Err(transaction::Error::ReferenceOutOfDate { full_name, actual, .. }) => {
            assert_eq!(full_name, "HEAD");
            assert_eq!(actual, target);
        }
        _ => unreachable!("unexpected result"),
    }
    Ok(())
}

#[test]
fn reference_with_create_only_must_not_exist_already_when_creating_it_if_the_value_does_not_match() -> crate::Result {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
    let head = store.loose_find("HEAD")?.expect("head exists already");
    let target = head.target;

    let res = store.transaction().prepare(
        Some(
            RefEdit {
                change: Change::Update {
                    log: LogChange::default(),
                    new: Target::Peeled(ObjectId::null_sha1()),
                    mode: Create::Only,
                },
                name: "HEAD".try_into()?,
                deref: false,
            }
            .clone(),
        ),
        Fail::Immediately,
    );
    match res {
        Err(transaction::Error::MustNotExist { full_name, actual, .. }) => {
            assert_eq!(full_name, "HEAD");
            assert_eq!(actual, target);
        }
        _ => unreachable!("unexpected result"),
    }
    Ok(())
}

#[test]
fn reference_with_create_only_must_not_exist_already_when_creating_it_unless_the_value_matches() -> crate::Result {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
    let head = store.loose_find("HEAD")?.expect("head exists already");
    let target = head.target;
    let previous_reflog_count = reflog_lines(&store, "HEAD")?.len();

    let edits = store
        .transaction()
        .prepare(
            Some(
                RefEdit {
                    change: Change::Update {
                        log: LogChange::default(),
                        new: target.clone(),
                        mode: Create::Only,
                    },
                    name: "HEAD".try_into()?,
                    deref: false,
                }
                .clone(),
            ),
            Fail::Immediately,
        )?
        .commit(&committer())?
        .0;

    assert_eq!(
        edits,
        vec![RefEdit {
            change: Change::Update {
                log: LogChange::default(),
                new: target.clone(),
                mode: Create::OrUpdate { previous: Some(target) },
            },
            name: "HEAD".try_into()?,
            deref: false,
        }]
    );

    assert_eq!(
        reflog_lines(&store, "HEAD")?.len(),
        previous_reflog_count,
        "no new reflog is actually added"
    );
    Ok(())
}

#[test]
fn cancellation_after_preparation_leaves_no_change() -> crate::Result {
    let (dir, store) = empty_store()?;

    let tx = store.transaction();

    assert_eq!(
        std::fs::read_dir(dir.path())?.count(),
        0,
        "nothing happens before preparation"
    );

    let tx = tx.prepare(
        Some(RefEdit {
            change: Change::Update {
                log: LogChange::default(),
                new: Target::Symbolic("refs/heads/main".try_into().unwrap()),
                mode: Create::Only,
            },
            name: "HEAD".try_into()?,
            deref: false,
        }),
        Fail::Immediately,
    )?;
    assert_eq!(std::fs::read_dir(dir.path())?.count(), 1, "the lock file was created");

    drop(tx);
    assert_eq!(std::fs::read_dir(dir.path())?.count(), 0, "everything vanished");
    Ok(())
}

#[test]
fn symbolic_head_missing_referent_then_update_referent() -> crate::Result {
    for reflog_writemode in &[WriteReflog::Normal, WriteReflog::Disable] {
        let (_keep, mut store) = empty_store()?;
        store.write_reflog = *reflog_writemode;
        let referent = "refs/heads/alt-main";
        assert!(store.loose_find(referent)?.is_none(), "the reference does not exist");
        let log_ignored = LogChange {
            mode: RefLog::AndReference,
            force_create_reflog: false,
            message: "ignored".into(),
        };
        let new_head_value = Target::Symbolic(referent.try_into().unwrap());
        let edits = store
            .transaction()
            .prepare(
                Some(RefEdit {
                    change: Change::Update {
                        log: log_ignored.clone(),
                        new: new_head_value.clone(),
                        mode: Create::Only,
                    },
                    name: "HEAD".try_into()?,
                    deref: false,
                }),
                Fail::Immediately,
            )?
            .commit(&committer())?
            .0;
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

        let head = store.loose_find_existing(edits[0].name.to_partial())?;
        assert_eq!(head.name.as_bstr(), "HEAD");
        assert_eq!(head.kind(), git_ref::Kind::Symbolic);
        assert_eq!(head.target.borrow().as_name(), Some(referent.as_bytes().as_bstr()));
        assert!(!head.log_exists(&store), "no reflog is written for symbolic ref");
        assert!(store.loose_find(referent)?.is_none(), "referent wasn't created");

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
            .transaction()
            .prepare(
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
            )?
            .commit(&committer())?
            .0;

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

        let head = store.loose_find_existing("HEAD")?;
        assert_eq!(
            head.kind(),
            git_ref::Kind::Symbolic,
            "head is still symbolic, not detached"
        );
        assert_eq!(
            head.target.borrow().as_name(),
            Some(referent.as_bytes().as_bstr()),
            "it still points to the referent"
        );

        let referent_ref = store.loose_find_existing(referent)?;
        assert_eq!(referent_ref.kind(), git_ref::Kind::Peeled, "referent is a peeled ref");
        assert_eq!(
            referent_ref.target.borrow().as_id(),
            Some(new_oid.as_ref()),
            "referent points to desired hash"
        );

        let mut buf = Vec::new();
        for ref_name in &["HEAD", referent] {
            match reflog_writemode {
                WriteReflog::Normal => {
                    let expected_line = log_line(ObjectId::null_sha1(), new_oid, "an actual change");
                    assert_eq!(reflog_lines(&store, *ref_name)?, vec![expected_line]);
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
/// Writing a peeled ref to which head points to doesn't update HEAD on the fly even though that might be what's would
/// be needed to keep the reflog consistent
fn write_reference_to_which_head_points_to_does_not_update_heads_reflog_even_though_it_should() -> crate::Result {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
    let head = store.loose_find_existing("HEAD")?;
    let referent = head.target.borrow().as_name().expect("symbolic ref").to_owned();
    let previous_head_reflog = reflog_lines(&store, "HEAD")?;

    let new_id = hex_to_id("01dd4e2a978a9f5bd773dae6da7aa4a5ac1cdbbc");
    let edits = store
        .transaction()
        .prepare(
            Some(RefEdit {
                change: Change::Update {
                    log: LogChange {
                        mode: RefLog::AndReference,
                        force_create_reflog: false,
                        message: "".into(),
                    },
                    mode: Create::OrUpdate {
                        previous: Some(Target::must_exist()),
                    },
                    new: Target::Peeled(new_id),
                },
                name: referent.as_bstr().try_into()?,
                deref: false,
            }),
            Fail::Immediately,
        )?
        .commit(&committer())?
        .0;

    assert_eq!(edits.len(), 1, "HEAD wasn't update");
    assert_eq!(
        edits,
        vec![RefEdit {
            change: Change::Update {
                log: LogChange {
                    mode: RefLog::AndReference,
                    force_create_reflog: false,
                    message: "".into(),
                },
                mode: Create::OrUpdate {
                    previous: Some(Target::Peeled(hex_to_id("02a7a22d90d7c02fb494ed25551850b868e634f0"))),
                },
                new: Target::Peeled(new_id),
            },
            name: referent.as_bstr().try_into()?,
            deref: false,
        }]
    );
    assert_eq!(
        reflog_lines(&store, "HEAD")?,
        previous_head_reflog,
        "nothing changed in the heads reflog"
    );

    let expected_line = log_line(hex_to_id("02a7a22d90d7c02fb494ed25551850b868e634f0"), new_id, "");
    assert_eq!(
        reflog_lines(&store, &referent.to_string())?
            .last()
            .expect("at least one line"),
        &expected_line,
        "referent line matches the expected one"
    );
    Ok(())
}
