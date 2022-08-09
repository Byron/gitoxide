use git_repository::ThreadSafeRepository;

#[test]
fn valid() -> crate::Result {
    let repo_path = git_testtools::scripted_fixture_repo_read_only("make_core_worktree_repo.sh")?.join("repo");
    let repo = ThreadSafeRepository::open_opts(repo_path, crate::restricted())?;

    assert_eq!(
        repo.workdir()
            .expect("repository has a workdir")
            .canonicalize()
            .expect("workdir path exists"),
        repo.git_dir()
            .canonicalize()
            .expect("git_dir path exists")
            .parent()
            .expect("parent always exists")
            .parent()
            .expect("parent always exists")
            .join("worktree"),
        "work_dir is set to core.worktree config value"
    );
    Ok(())
}
