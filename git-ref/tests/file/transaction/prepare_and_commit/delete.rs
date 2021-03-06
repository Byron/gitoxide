use crate::file::transaction::prepare_and_commit::committer;
use crate::file::{store_writable, transaction::prepare_and_commit::empty_store};
use git_lock::acquire::Fail;
use git_ref::{
    mutable::Target,
    transaction::{Change, RefEdit, RefLog},
};
use std::convert::TryInto;

#[test]
fn delete_a_ref_which_is_gone_succeeds() -> crate::Result {
    let (_keep, store) = empty_store()?;
    let edits = store
        .transaction(
            Some(RefEdit {
                change: Change::Delete {
                    previous: None,
                    log: RefLog::AndReference,
                },
                name: "DOES_NOT_EXIST".try_into()?,
                deref: false,
            }),
            Fail::Immediately,
        )
        .commit(&committer())?;
    assert_eq!(edits.len(), 1);
    Ok(())
}

#[test]
fn delete_a_ref_which_is_gone_but_must_exist_fails() -> crate::Result {
    let (_keep, store) = empty_store()?;
    let res = store
        .transaction(
            Some(RefEdit {
                change: Change::Delete {
                    previous: Some(Target::must_exist()),
                    log: RefLog::AndReference,
                },
                name: "DOES_NOT_EXIST".try_into()?,
                deref: false,
            }),
            Fail::Immediately,
        )
        .commit(&committer());
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
    let head = store.find_one_existing("HEAD")?;
    assert!(head.log_exists().unwrap());
    let _main = store.find_one_existing("main")?;

    let edits = store
        .transaction(
            Some(RefEdit {
                change: Change::Delete {
                    previous: Some(Target::must_exist()),
                    log: RefLog::AndReference,
                },
                name: head.name().into(),
                deref: false,
            }),
            Fail::Immediately,
        )
        .commit(&committer())?;

    assert_eq!(
        edits,
        vec![RefEdit {
            change: Change::Delete {
                previous: Some(Target::Symbolic("refs/heads/main".try_into()?)),
                log: RefLog::AndReference,
            },
            name: head.name().into(),
            deref: false
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
fn delete_ref_with_incorrect_previous_value_fails() {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh").unwrap();
    let head = store.find_one_existing("HEAD").unwrap();
    assert!(head.log_exists().unwrap());

    let err = store
        .transaction(
            Some(RefEdit {
                change: Change::Delete {
                    previous: Some(Target::Symbolic("refs/heads/main".try_into().unwrap())),
                    log: RefLog::Only,
                },
                name: head.name().into(),
                deref: true,
            }),
            Fail::Immediately,
        )
        .commit(&committer())
        .expect_err("mismatch is detected");

    assert_eq!(err.to_string(), "The reference 'refs/heads/main' should have content ref: refs/heads/main, actual content was 02a7a22d90d7c02fb494ed25551850b868e634f0");
    // everything stays as is
    let head = store.find_one_existing("HEAD").unwrap();
    assert!(head.log_exists().unwrap());
    let main = store.find_one_existing("main").expect("referent still exists");
    assert!(main.log_exists().unwrap());
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
                    log: RefLog::Only,
                },
                name: head.name().into(),
                deref: false,
            }),
            Fail::Immediately,
        )
        .commit(&committer())?;

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
fn delete_reflog_only_of_symbolic_with_deref() -> crate::Result {
    let (_keep, store) = store_writable("make_repo_for_reflog.sh")?;
    let head = store.find_one_existing("HEAD")?;
    assert!(head.log_exists()?);

    let edits = store
        .transaction(
            Some(RefEdit {
                change: Change::Delete {
                    previous: Some(Target::must_exist()),
                    log: RefLog::Only,
                },
                name: head.name().into(),
                deref: true,
            }),
            Fail::Immediately,
        )
        .commit(&committer())?;

    assert_eq!(edits.len(), 2);
    let head = store.find_one_existing("HEAD")?;
    assert!(!head.log_exists()?);
    let main = store.find_one_existing("main").expect("referent still exists");
    assert!(!main.log_exists()?, "log is removed");
    assert_eq!(
        main.target(),
        head.peel_one_level().expect("a symref")?.target(),
        "head points to main"
    );
    Ok(())
}

#[test]
/// Based on https://github.com/git/git/blob/master/refs/files-backend.c#L514:L515
fn delete_broken_ref_that_must_exist_fails_as_it_is_no_valid_ref() {
    let (_keep, store) = empty_store().unwrap();
    std::fs::write(store.base.join("HEAD"), &b"broken").unwrap();
    assert!(store.find_one("HEAD").is_err(), "the ref is truly broken");

    let err = store
        .transaction(
            Some(RefEdit {
                change: Change::Delete {
                    previous: Some(Target::must_exist()),
                    log: RefLog::AndReference,
                },
                name: "HEAD".try_into().unwrap(),
                deref: true,
            }),
            Fail::Immediately,
        )
        .commit(&committer())
        .expect_err("if refs must exist they must be readable too");
    assert_eq!(
        err.to_string(),
        "The reference 'HEAD' for deletion did not exist or could not be parsed"
    );
}

#[test]
/// Based on https://github.com/git/git/blob/master/refs/files-backend.c#L514:L515
fn delete_broken_ref_that_may_not_exist_works_even_in_deref_mode() -> crate::Result {
    let (_keep, store) = empty_store()?;
    std::fs::write(store.base.join("HEAD"), &b"broken")?;
    assert!(store.find_one("HEAD").is_err(), "the ref is truly broken");

    let edits = store
        .transaction(
            Some(RefEdit {
                change: Change::Delete {
                    previous: None,
                    log: RefLog::AndReference,
                },
                name: "HEAD".try_into()?,
                deref: true,
            }),
            Fail::Immediately,
        )
        .commit(&committer())?;

    assert!(store.find_one("HEAD")?.is_none(), "the ref was deleted");
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
        assert!(store.find_one_existing("HEAD")?.log_exists()?,);
        let edits = store
            .transaction(
                Some(RefEdit {
                    change: Change::Delete {
                        previous: None,
                        log: RefLog::Only,
                    },
                    name: "HEAD".try_into()?,
                    deref: false,
                }),
                Fail::Immediately,
            )
            .commit(&committer())?;
        assert_eq!(edits.len(), 1);
        assert!(!store.find_one_existing("HEAD")?.log_exists()?, "log was deleted");
    }
    Ok(())
}
