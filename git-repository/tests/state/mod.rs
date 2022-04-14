use crate::{repo, Result};
use anyhow::anyhow;
use git_repository::{bstr::ByteSlice, RepositoryState};

// Can we identify that a cherry pick operation is in progress
#[test]
fn cherry_pick() -> Result {
    let repo = repo("make_cherry_pick_repo.sh").map(|r| r.to_thread_local())?;

    let head = repo.head()?;
    let head_name = head
        .referent_name()
        .ok_or_else(|| anyhow!("detached head?"))?
        .shorten()
        .to_str()?;
    assert_eq!("master", head_name);

    assert_eq!(Some(RepositoryState::CherryPick), repo.in_progress_operation());

    Ok(())
}

// Can we identify that we're in the middle of an interactive rebase?
#[test]
fn rebase_interactive() -> Result {
    let repo = repo("make_rebase_i_repo.sh").map(|r| r.to_thread_local())?;

    let head = repo.head()?;
    // TODO: Get rebase head/target
    let head_name = head.referent_name();
    assert!(head_name.is_none());

    assert_eq!(Some(RepositoryState::RebaseInteractive), repo.in_progress_operation());

    Ok(())
}

// Can we identify a revert operation when we see it?
#[test]
fn revert() -> Result {
    let repo = repo("make_revert_repo.sh").map(|r| r.to_thread_local())?;

    let head = repo.head()?;
    let head_name = head
        .referent_name()
        .ok_or_else(|| anyhow!("detached head?"))?
        .shorten()
        .to_str()?;
    assert_eq!("master", head_name);

    assert_eq!(Some(RepositoryState::Revert), repo.in_progress_operation());

    Ok(())
}
