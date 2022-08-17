mod submodules {
    use git_repository as git;
    use std::path::Path;

    #[test]
    #[ignore]
    fn by_their_worktree_checkout_and_git_modules_dir() -> crate::Result {
        let parent_repo = Path::new("with-submodules");
        let modules = parent_repo.join(".git").join("modules");
        for module in ["m1", "dir/m1"] {
            let submodule_m1_workdir = parent_repo.join(module);
            let submodule_m1_gitdir = modules.join(module);

            for discover_dir in [
                submodule_m1_workdir.clone(),
                submodule_m1_workdir.join("subdir"),
                submodule_m1_gitdir.clone(),
            ] {
                let repo = discover_repo(discover_dir)?;
                assert_eq!(repo.work_dir().expect("non-bare"), submodule_m1_workdir);
                assert_eq!(repo.git_dir(), submodule_m1_gitdir);

                let repo = git::open_opts(repo.work_dir().expect("non-bare"), git::open::Options::isolated())?;
                assert_eq!(repo.work_dir().expect("non-bare"), submodule_m1_workdir);
                assert_eq!(repo.git_dir(), submodule_m1_gitdir);
            }
        }
        Ok(())
    }

    fn discover_repo(name: impl AsRef<Path>) -> crate::Result<git::Repository> {
        let dir = git_testtools::scripted_fixture_repo_read_only("make_submodules.sh")?;
        let repo_dir = dir.join(name);
        Ok(git::ThreadSafeRepository::discover_opts(
            repo_dir,
            Default::default(),
            git_sec::trust::Mapping {
                full: crate::restricted(),
                reduced: crate::restricted(),
            },
        )?
        .to_thread_local())
    }
}
