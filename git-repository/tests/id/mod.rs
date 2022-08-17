use std::cmp::Ordering;

use git_repository as git;
use git_repository::prelude::ObjectIdExt;
use git_testtools::hex_to_id;

#[test]
fn prefix() -> crate::Result {
    let (repo, worktree_dir) = crate::repo_rw("make_repo_with_fork_and_dates.sh")?;
    let id = hex_to_id("288e509293165cb5630d08f4185bdf2445bf6170").attach(&repo);
    let prefix = id.shorten()?;
    assert_eq!(prefix.cmp_oid(&id), Ordering::Equal);
    assert_eq!(prefix.hex_len(), 7, "preconfigured via core.abbrev default value");

    // TODO: do this in-memory (with or without writing to disk)
    assert!(
        git_testtools::run_git(worktree_dir.path(), &["config", "--int", "core.abbrev", "5"])?.success(),
        "set core abbrev value successfully"
    );

    let repo = git_repository::open(worktree_dir.path()).unwrap();
    let id = id.detach().attach(&repo);
    let prefix = id.shorten()?;
    assert_eq!(prefix.cmp_oid(&id), Ordering::Equal);
    assert_eq!(prefix.hex_len(), 5, "preconfigured via core.abbrev in the repo file");

    assert!(
        git_testtools::run_git(worktree_dir.path(), &["config", "core.abbrev", ""])?.success(),
        "set core abbrev value to empty successfully"
    );

    assert!(
        matches!(
            git_repository::open_opts(worktree_dir.path(), git::open::Options::isolated().strict_config(true))
                .unwrap_err(),
            git::open::Error::Config(git::config::Error::EmptyValue { .. })
        ),
        "an empty core.abbrev fails the open operation in strict config mode, emulating git behaviour"
    );
    assert!(
        git_repository::open(worktree_dir.path()).is_ok(),
        "By default gitoxide acts like `libgit2` here and we prefer to be lenient when possible"
    );
    Ok(())
}

mod ancestors {
    use git_traverse::commit;

    #[test]
    fn all() -> crate::Result {
        let repo = crate::repo("make_repo_with_fork_and_dates.sh")?.to_thread_local();
        let head = repo.head()?.into_fully_peeled_id().expect("born")?;
        let commits_graph_order = head.ancestors().all()?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(commits_graph_order.len(), 4, "need a specific amount of commits");

        let commits_by_commit_date = head
            .ancestors()
            .sorting(commit::Sorting::ByCommitTimeNewestFirst)
            .all()?
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(
            commits_by_commit_date.len(),
            4,
            "need a specific amount of commits, ordering doesn't affect that"
        );
        assert_ne!(
            commits_by_commit_date, commits_graph_order,
            "these are ordered differently"
        );

        assert_eq!(
            head.ancestors().first_parent_only().all()?.count(),
            3,
            "It skips merges this way."
        );
        Ok(())
    }
}
