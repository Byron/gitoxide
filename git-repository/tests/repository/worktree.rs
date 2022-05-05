use git_repository as git;

#[test]
fn from_bare_parent_repo() {
    let dir = git_testtools::scripted_fixture_repo_read_only_with_args("make_worktree_repo.sh", ["bare"]).unwrap();
    let repo = git::open(dir.join("repo.git")).unwrap();

    assert!(repo.is_bare());
}

#[test]
fn from_nonbare_parent_repo() {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_worktree_repo.sh").unwrap();
    let repo = git::open(dir.join("repo")).unwrap();

    assert!(!repo.is_bare());
}
