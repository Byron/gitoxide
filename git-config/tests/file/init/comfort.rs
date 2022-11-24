use git_config::source;
use git_testtools::Env;
use serial_test::serial;

#[test]
fn from_globals() {
    let config = git_config::File::from_globals().unwrap();
    assert!(config.sections().all(|section| {
        let kind = section.meta().source.kind();
        kind != source::Kind::Repository && kind != source::Kind::Override
    }));
}

#[test]
#[serial]
fn from_environment_overrides() {
    let config = git_config::File::from_environment_overrides().unwrap();
    assert!(config.is_void());
}

#[test]
#[serial]
fn from_git_dir() -> crate::Result {
    let worktree_dir = git_testtools::scripted_fixture_repo_read_only("make_config_repo.sh")?;
    let git_dir = worktree_dir.join(".git");
    let worktree_dir = worktree_dir.canonicalize()?;
    let _env = Env::new()
        .set(
            "GIT_CONFIG_SYSTEM",
            worktree_dir.join("system.config").display().to_string(),
        )
        .set("HOME", worktree_dir.display().to_string())
        .set("GIT_CONFIG_COUNT", "1")
        .set("GIT_CONFIG_KEY_0", "include.path")
        .set(
            "GIT_CONFIG_VALUE_0",
            worktree_dir.join("c.config").display().to_string(),
        );

    let config = git_config::File::from_git_dir(git_dir)?;
    assert_eq!(
        config.string("a", None, "local").expect("present").as_ref(),
        "value",
        "a value from the local repo configuration"
    );
    assert_eq!(config.string_by_key("a.local").expect("present").as_ref(), "value",);
    assert_eq!(
        config.string("a", None, "local-include").expect("present").as_ref(),
        "from-a.config",
        "an override from a local repo include"
    );
    assert_eq!(
        config.string("a", None, "system").expect("present").as_ref(),
        "from-system.config",
        "system configuration can be overridden with GIT_CONFIG_SYSTEM"
    );
    assert_eq!(
        config.string("a", None, "system-override").expect("present").as_ref(),
        "from-b.config",
        "globals resolve their includes"
    );
    assert_eq!(
        config.string("a", None, "user").expect("present").as_ref(),
        "from-user.config",
        "per-user configuration"
    );
    assert_eq!(
        config.string("env", None, "override").expect("present").as_ref(),
        "from-c.config",
        "environment includes are resolved"
    );

    // on CI this file actually exists in xdg home and our values aren't present
    if !(cfg!(unix) && git_testtools::is_ci::cached()) {
        assert_eq!(
            config.string("a", None, "git").expect("present").as_ref(),
            "git-application",
            "we load the XDG directories, based on the HOME fallback"
        );
    }
    Ok(())
}

#[test]
#[serial]
fn from_git_dir_with_worktree_extension() -> crate::Result {
    let git_dir = git_testtools::scripted_fixture_repo_read_only("config_with_worktree_extension.sh")?
        .join("main-worktree")
        .join(".git");
    let config = git_config::File::from_git_dir(git_dir)?;

    assert_eq!(
        config
            .string("extensions", None, "worktreeConfig")
            .expect("extension present")
            .as_ref(),
        "true"
    );
    assert_eq!(
        config
            .string("worktree", None, "override")
            .expect("section present")
            .as_ref(),
        "set in the main worktree"
    );

    Ok(())
}
