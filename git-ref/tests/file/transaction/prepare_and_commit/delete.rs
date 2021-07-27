use crate::file::transaction::prepare_and_commit::committer;
use crate::file::{store_writable, transaction::prepare_and_commit::empty_store};
use git_lock::acquire::Fail;
use git_ref::{
    mutable::Target,
    transaction::{Change, RefEdit, RefLog},
};
use git_testtools::hex_to_id;
use std::convert::TryInto;

#[test]
fn delete_a_ref_which_is_gone_succeeds() -> crate::Result {
    let (_keep, store) = empty_store()?;
    let edits = store
        .transaction()
        .prepare(
            Some(RefEdit {
                change: Change::Delete {
                    previous: None,
                    log: RefLog::AndReference,
                },
                name: "DOES_NOT_EXIST".try_into()?,
                deref: false,
            }),
            Fail::Immediately,
        )?
        .commit(&committer())?
        .0;
    assert_eq!(edits.len(), 1);
    Ok(())
}

#[test]
fn delete_a_ref_which_is_gone_but_must_exist_fails() -> crate::Result {
    let (_keep, store) = empty_store()?;
    let res = store.transaction().prepare(
        Some(RefEdit {
            change: Change::Delete {
                previous: Some(Target::must_exist()),
                log: RefLog::AndReference,
            },
            name: "DOES_NOT_EXIST".try_into()?,
            deref: false,
        }),
        Fail::Immediately,
    );
    match res {
        Ok(_) => unreachable!("must exist, but it doesn't actually exist"),
        Err(err) => assert_eq!(
            err.to_string(),
            "The reference 'DOES_NOT_EXIST' for deletion did not exist or could not be parsed"
        ),
    }
    Ok(())
}

#[test]
fn delete_ref_and_reflog_on_symbolic_no_deref() -> crate::Result {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
    let head = store.loose_find_existing("HEAD")?;
    assert!(head.log_exists(&store));
    let _main = store.loose_find_existing("main")?;

    let edits = store
        .transaction()
        .prepare(
            Some(RefEdit {
                change: Change::Delete {
                    previous: Some(Target::must_exist()),
                    log: RefLog::AndReference,
                },
                name: head.name.clone(),
                deref: false,
            }),
            Fail::Immediately,
        )?
        .commit(&committer())?
        .0;

    assert_eq!(
        edits,
        vec![RefEdit {
            change: Change::Delete {
                previous: Some(Target::Symbolic("refs/heads/main".try_into()?)),
                log: RefLog::AndReference,
            },
            name: head.name,
            deref: false
        }],
        "the previous value was updated with the actual one"
    );
    assert!(
        store.reflog_iter_rev("HEAD", &mut [0u8; 128])?.is_none(),
        "reflog was deleted"
    );
    assert!(store.loose_find("HEAD")?.is_none(), "ref was deleted");
    assert!(store.loose_find("main")?.is_some(), "referent still exists");
    Ok(())
}

#[test]
fn delete_ref_with_incorrect_previous_value_fails() -> crate::Result {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
    let head = store.loose_find_existing("HEAD")?;
    assert!(head.log_exists(&store));

    let res = store.transaction().prepare(
        Some(RefEdit {
            change: Change::Delete {
                previous: Some(Target::Symbolic("refs/heads/main".try_into()?)),
                log: RefLog::Only,
            },
            name: head.name.clone(),
            deref: true,
        }),
        Fail::Immediately,
    );

    match res {
        Err(err) => {
            assert_eq!(err.to_string(), "The reference 'refs/heads/main' should have content ref: refs/heads/main, actual content was 02a7a22d90d7c02fb494ed25551850b868e634f0");
        }
        Ok(_) => unreachable!("must be err"),
    }
    // everything stays as is
    let head = store.loose_find_existing("HEAD")?;
    assert!(head.log_exists(&store));
    let main = store.loose_find_existing("main").expect("referent still exists");
    assert!(main.log_exists(&store));
    Ok(())
}

#[test]
fn delete_reflog_only_of_symbolic_no_deref() -> crate::Result {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
    let head = store.loose_find_existing("HEAD")?;
    assert!(head.log_exists(&store));

    let edits = store
        .transaction()
        .prepare(
            Some(RefEdit {
                change: Change::Delete {
                    previous: Some(Target::Symbolic("refs/heads/main".try_into()?)),
                    log: RefLog::Only,
                },
                name: head.name,
                deref: false,
            }),
            Fail::Immediately,
        )?
        .commit(&committer())?
        .0;

    assert_eq!(edits.len(), 1);
    let head = store.loose_find_existing("HEAD")?;
    assert!(!head.log_exists(&store));
    let main = store.loose_find_existing("main").expect("referent still exists");
    assert!(main.log_exists(&store), "log is untouched, too");
    assert_eq!(
        main.target,
        head.peel_one_level(&store, None).expect("a symref")?.target(),
        "head points to main"
    );
    Ok(())
}

#[test]
fn delete_reflog_only_of_symbolic_with_deref() -> crate::Result {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
    let head = store.loose_find_existing("HEAD")?;
    assert!(head.log_exists(&store));

    let edits = store
        .transaction()
        .prepare(
            Some(RefEdit {
                change: Change::Delete {
                    previous: Some(Target::must_exist()),
                    log: RefLog::Only,
                },
                name: head.name,
                deref: true,
            }),
            Fail::Immediately,
        )?
        .commit(&committer())?
        .0;

    assert_eq!(edits.len(), 2);
    let head = store.loose_find_existing("HEAD")?;
    assert!(!head.log_exists(&store));
    let main = store.loose_find_existing("main").expect("referent still exists");
    assert!(!main.log_exists(&store), "log is removed");
    assert_eq!(
        main.target,
        head.peel_one_level(&store, None).expect("a symref")?.target(),
        "head points to main"
    );
    Ok(())
}

#[test]
/// Based on https://github.com/git/git/blob/master/refs/files-backend.c#L514:L515
fn delete_broken_ref_that_must_exist_fails_as_it_is_no_valid_ref() -> crate::Result {
    let (_keep, store) = empty_store()?;
    std::fs::write(store.base.join("HEAD"), &b"broken")?;
    assert!(store.loose_find("HEAD").is_err(), "the ref is truly broken");

    let res = store.transaction().prepare(
        Some(RefEdit {
            change: Change::Delete {
                previous: Some(Target::must_exist()),
                log: RefLog::AndReference,
            },
            name: "HEAD".try_into()?,
            deref: true,
        }),
        Fail::Immediately,
    );
    match res {
        Err(err) => {
            assert_eq!(
                err.to_string(),
                "The reference 'HEAD' for deletion did not exist or could not be parsed"
            );
        }
        Ok(_) => unreachable!("expected error"),
    }
    Ok(())
}

#[test]
/// Based on https://github.com/git/git/blob/master/refs/files-backend.c#L514:L515
fn delete_broken_ref_that_may_not_exist_works_even_in_deref_mode() -> crate::Result {
    let (_keep, store) = empty_store()?;
    std::fs::write(store.base.join("HEAD"), &b"broken")?;
    assert!(store.loose_find("HEAD").is_err(), "the ref is truly broken");

    let edits = store
        .transaction()
        .prepare(
            Some(RefEdit {
                change: Change::Delete {
                    previous: None,
                    log: RefLog::AndReference,
                },
                name: "HEAD".try_into()?,
                deref: true,
            }),
            Fail::Immediately,
        )?
        .commit(&committer())?
        .0;

    assert!(store.loose_find("HEAD")?.is_none(), "the ref was deleted");
    assert_eq!(
        edits,
        vec![RefEdit {
            change: Change::Delete {
                previous: None,
                log: RefLog::AndReference,
            },
            name: "HEAD".try_into()?,
            deref: false,
        }]
    );
    Ok(())
}

#[test]
fn store_write_mode_has_no_effect_and_reflogs_are_always_deleted() -> crate::Result {
    for reflog_writemode in &[git_ref::file::WriteReflog::Normal, git_ref::file::WriteReflog::Disable] {
        let (_keep, mut store) = store_writable("make_repo_for_reflog.sh")?;
        store.write_reflog = *reflog_writemode;
        assert!(store.loose_find_existing("HEAD")?.log_exists(&store));
        let edits = store
            .transaction()
            .prepare(
                Some(RefEdit {
                    change: Change::Delete {
                        previous: None,
                        log: RefLog::Only,
                    },
                    name: "HEAD".try_into()?,
                    deref: false,
                }),
                Fail::Immediately,
            )?
            .commit(&committer())?
            .0;
        assert_eq!(edits.len(), 1);
        assert!(
            !store.loose_find_existing("HEAD")?.log_exists(&store),
            "log was deleted"
        );
    }
    Ok(())
}

#[test]
fn packed_refs_are_consulted_when_determining_previous_value_of_ref_to_be_deleted_and_are_deleted_from_packed_ref_file()
{
    let (_keep, store) = store_writable("make_packed_ref_repository.sh").unwrap();
    assert!(
        store.loose_find("main").unwrap().is_none(),
        "no loose main available, it's packed"
    );

    let old_id = hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03");
    let (edits, packed) = store
        .transaction()
        .prepare(
            Some(RefEdit {
                change: Change::Delete {
                    previous: Some(Target::Peeled(old_id)),
                    log: RefLog::AndReference,
                },
                name: "refs/heads/main".try_into().unwrap(),
                deref: false,
            }),
            git_lock::acquire::Fail::Immediately,
        )
        .unwrap()
        .commit(&committer())
        .unwrap();

    assert_eq!(edits.len(), 1, "an edit was performed in the packed refs store");
    let packed = packed.expect("packed ref present");
    assert!(packed.find("main").unwrap().is_none(), "no main present after deletion");
}

#[test]
#[ignore]
fn a_loose_ref_with_old_value_check_and_outdated_packed_refs_value_deletes_both_refs() {
    todo!("use overlay repository as baseline and delete shadowed value by name")
}
