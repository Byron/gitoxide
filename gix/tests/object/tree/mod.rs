use crate::util::{named_repo, named_subrepo_opts};

#[cfg(all(feature = "blob-diff", feature = "revision"))]
mod diff;

#[test]
fn find_entry() -> crate::Result {
    let repo = named_repo("make_basic_repo.sh")?;
    let tree = repo.head_commit()?.tree()?;
    assert_eq!(tree.find_entry("this").expect("present").filename(), "this");

    assert!(tree.find_entry("not there").is_none());
    Ok(())
}

#[test]
fn lookup_entry_by_path() -> crate::Result {
    let repo = named_subrepo_opts("make_worktree_repo.sh", "repo", gix::open::Options::isolated())?;
    let tree = repo.head_commit()?.tree()?;
    assert_eq!(
        tree.lookup_entry_by_path("dir/c", &mut Vec::new())?
            .expect("present")
            .filename(),
        "c"
    );
    Ok(())
}
