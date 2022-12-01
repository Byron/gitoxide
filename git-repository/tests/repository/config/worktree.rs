use git_repository::open;

#[test]
fn with_worktree_configs() -> git_testtools::Result {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let fixture_dir = git_testtools::scripted_fixture_repo_read_only("make_worktree_repo_with_configs.sh").unwrap();

    let base = open(fixture_dir.join("repo")).unwrap();
    let base_config = base.config_snapshot();

    assert_eq!(base.work_dir(), Some(fixture_dir.join("repo").as_path()));
    assert_eq!(base.git_dir(), fixture_dir.join("repo/.git"));
    assert_eq!(base.common_dir(), fixture_dir.join("repo/.git"));

    assert_eq!(
        base_config.string("worktree.setting").expect("exists").to_string(),
        "set in the main worktree"
    );
    assert_eq!(
        base_config.string("shared.setting").expect("exists").to_string(),
        "set in the shared config"
    );

    let wt1 = open(fixture_dir.join("wt-1")).unwrap();
    let wt1_config = wt1.config_snapshot();
    assert_eq!(wt1.work_dir(), Some(fixture_dir.join("wt-1").as_path()));
    assert_eq!(
        wt1.git_dir(),
        manifest_dir.join(&fixture_dir).join("repo/.git/worktrees/wt-1")
    );
    assert_eq!(
        wt1.common_dir(),
        manifest_dir.join(&fixture_dir).join("repo/.git/worktrees/wt-1/../..")
    );

    assert_eq!(
        wt1_config.string("worktree.setting").expect("exists").to_string(),
        "set in wt-1"
    );
    assert_eq!(
        wt1_config.string("shared.setting").expect("exists").to_string(),
        "set in the shared config"
    );

    let wt2 = open(fixture_dir.join("wt-2")).unwrap();
    let wt2_config = wt2.config_snapshot();
    assert_eq!(wt2.work_dir(), Some(fixture_dir.join("wt-2").as_path()));
    assert_eq!(
        wt2.git_dir(),
        manifest_dir.join(&fixture_dir).join("repo/.git/worktrees/wt-2")
    );
    assert_eq!(
        wt2.common_dir(),
        manifest_dir.join(&fixture_dir).join("repo/.git/worktrees/wt-2/../..")
    );

    assert_eq!(
        wt2_config.string("worktree.setting").expect("exists").to_string(),
        "set in wt-2"
    );
    assert_eq!(
        wt2_config.string("shared.setting").expect("exists").to_string(),
        "set in the shared config"
    );

    Ok(())
}
