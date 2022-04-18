use crate::{named_repo, Result};
use git_repository as git;

#[test]
fn bisect() -> Result {
    let repo = named_repo("make_bisect_repo.sh")?;

    let head = repo.head()?;
    let head_name = head.referent_name().expect("no detached head").shorten();
    assert_eq!(head_name, "main");

    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::Bisect));

    Ok(())
}

#[test]
fn cherry_pick() -> Result {
    let repo = named_repo("make_cherry_pick_repo.sh")?;

    let head = repo.head()?;
    let head_name = head.referent_name().expect("no detached head").shorten();
    assert_eq!(head_name, "main");

    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::CherryPick));
    Ok(())
}

#[test]
fn cherry_pick_sequence() -> Result {
    let repo = named_repo("make_cherry_pick_sequence_repo.sh")?;

    let head = repo.head()?;
    let head_name = head.referent_name().expect("no detached head").shorten();
    assert_eq!(head_name, "main");

    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::CherryPickSequence));

    Ok(())
}

#[test]
fn merge() -> Result {
    let repo = named_repo("make_merge_repo.sh")?;

    let head = repo.head()?;
    let head_name = head.referent_name().expect("no detached head").shorten();
    assert_eq!(head_name, "main");

    assert_eq!(
        repo.in_progress_operation(),
        Some(git::state::InProgress::Merge)
    );

    Ok(())
}

#[test]
fn rebase_interactive() -> Result {
    let repo = named_repo("make_rebase_i_repo.sh")?;

    let head = repo.head()?;
    assert!(head.is_detached());
    assert_eq!(
        repo.in_progress_operation(),
        Some(git::state::InProgress::RebaseInteractive)
    );

    Ok(())
}

#[test]
fn revert() -> Result {
    let repo = named_repo("make_revert_repo.sh")?;

    let head = repo.head()?;
    let head_name = head.referent_name().expect("no detached head").shorten();
    assert_eq!(head_name, "main");

    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::Revert));

    Ok(())
}

#[test]
fn revert_sequence() -> Result {
    let repo = named_repo("make_revert_sequence_repo.sh")?;

    let head = repo.head()?;
    let head_name = head.referent_name().expect("no detached head").shorten();
    assert_eq!(head_name, "main");

    assert_eq!(repo.in_progress_operation(), Some(git::state::InProgress::RevertSequence));

    Ok(())
}
