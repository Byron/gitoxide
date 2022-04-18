use crate::{named_repo, Result};
use git_repository::RepositoryState;

#[test]
fn cherry_pick() -> Result {
    let repo = named_repo("make_cherry_pick_repo.sh")?;

    let head = repo.head()?;
    let head_name = head.referent_name().expect("no detached head").shorten();
    assert_eq!(head_name, "main");

    assert_eq!(repo.in_progress_operation(), Some(RepositoryState::CherryPick));
    Ok(())
}

#[test]
fn rebase_interactive() -> Result {
    let repo = named_repo("make_rebase_i_repo.sh")?;

    let head = repo.head()?;
    assert!(head.is_detached());
    assert_eq!(repo.in_progress_operation(), Some(RepositoryState::RebaseInteractive));

    Ok(())
}

#[test]
fn revert() -> Result {
    let repo = named_repo("make_revert_repo.sh")?;

    let head = repo.head()?;
    let head_name = head.referent_name().expect("no detached head").shorten();
    assert_eq!(head_name, "main");

    assert_eq!(Some(RepositoryState::Revert), repo.in_progress_operation());

    Ok(())
}
