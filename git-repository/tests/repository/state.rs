use crate::{named_repo, Result};
use git_repository as git;

#[test]
fn apply_mailbox() -> Result {
    let repo = named_repo("make_am_repo.sh")?;

    assert_eq!(repo.head_name()?.unwrap().shorten(), "main");
    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::ApplyMailbox));
    Ok(())
}

#[test]
fn bisect() -> Result {
    let repo = named_repo("make_bisect_repo.sh")?;

    assert_eq!(repo.head_name()?.unwrap().shorten(), "main");
    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::Bisect));

    Ok(())
}

#[test]
fn cherry_pick() -> Result {
    let repo = named_repo("make_cherry_pick_repo.sh")?;

    assert_eq!(repo.head_name()?.unwrap().shorten(), "main");
    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::CherryPick));
    Ok(())
}

#[test]
fn cherry_pick_sequence() -> Result {
    let repo = named_repo("make_cherry_pick_sequence_repo.sh")?;

    assert_eq!(repo.head_name()?.unwrap().shorten(), "main");
    assert_eq!(
        repo.in_progress_operation(),
        Some(git::state::InProgress::CherryPickSequence)
    );

    Ok(())
}

#[test]
fn merge() -> Result {
    let repo = named_repo("make_merge_repo.sh")?;

    assert_eq!(repo.head_name()?.unwrap().shorten(), "main");
    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::Merge));

    Ok(())
}

#[test]
fn rebase_interactive() -> Result {
    let repo = named_repo("make_rebase_i_repo.sh")?;

    assert!(repo.head()?.is_detached());
    assert_eq!(
        repo.in_progress_operation(),
        Some(git::state::InProgress::RebaseInteractive)
    );

    Ok(())
}

#[test]
fn revert() -> Result {
    let repo = named_repo("make_revert_repo.sh")?;

    assert_eq!(repo.head_name()?.unwrap().shorten(), "main");
    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::Revert));

    Ok(())
}

#[test]
fn revert_sequence() -> Result {
    let repo = named_repo("make_revert_sequence_repo.sh")?;

    assert_eq!(repo.head_name()?.unwrap().shorten(), "main");
    assert_eq!(
        repo.in_progress_operation(),
        Some(git::state::InProgress::RevertSequence)
    );

    Ok(())
}
