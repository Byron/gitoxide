use crate::file::transaction::prepare_and_commit::empty_store;
use bstr::ByteSlice;
use git_lock::acquire::Fail;
use git_ref::{
    mutable::Target,
    transaction::{Change, RefEdit, RefLog},
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
        let (_keep, store) = empty_store(*reflog_writemode)?;
        let referent = "refs/heads/alt-main";
        assert!(store.find_one(referent)?.is_none(), "the reference does not exist");
        let t = store.transaction(
            Some(RefEdit {
                change: Change::Update {
                    mode: RefLog::AndReference,
                    force_create_reflog: false,
                    new: Target::Symbolic(referent.try_into()?),
                    previous: None, // TODO: check failure if it doesn't exist
                },
                name: "HEAD".try_into()?,
                deref: false,
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
