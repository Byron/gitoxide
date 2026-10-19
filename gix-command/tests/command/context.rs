use gix_command::Context;

fn winfix(expected: impl Into<String>) -> String {
    // Unclear why it's not debug-printing the env on windows.
    if cfg!(windows) { "\"\"".into() } else { expected.into() }
}

#[test]
fn git_dir_sets_env_only() -> gix_testtools::TestResult {
    let dir = gix_testtools::tempfile::TempDir::new()?;
    let path = gix_path::realpath(dir.path())?;
    let ctx = Context {
        git_dir: Some(path.clone()),
        ..Default::default()
    };
    let cmd = std::process::Command::try_from(gix_command::prepare("").with_context(ctx))?;
    assert_eq!(
        cmd.get_envs().collect::<Vec<_>>(),
        [(std::ffi::OsStr::new("GIT_DIR"), Some(path.as_os_str()))],
        "the absolute Git directory is passed through unchanged"
    );
    assert_eq!(cmd.get_current_dir(), None, "context only sets the environment");
    Ok(())
}

#[test]
fn worktree_dir_sets_env_only() -> gix_testtools::TestResult {
    let dir = gix_testtools::tempfile::TempDir::new()?;
    let path = gix_path::realpath(dir.path())?;
    let ctx = Context {
        worktree_dir: Some(path.clone()),
        ..Default::default()
    };
    let cmd = std::process::Command::try_from(gix_command::prepare("").with_context(ctx))?;
    assert_eq!(
        cmd.get_envs().collect::<Vec<_>>(),
        [(std::ffi::OsStr::new("GIT_WORK_TREE"), Some(path.as_os_str()))],
        "the absolute worktree directory is passed through unchanged"
    );
    assert_eq!(cmd.get_current_dir(), None, "context only sets the environment");
    Ok(())
}

#[test]
fn no_replace_objects_sets_env_only() -> gix_testtools::TestResult {
    for value in [false, true] {
        let expected = usize::from(value);
        let ctx = Context {
            no_replace_objects: Some(value),
            ..Default::default()
        };
        let cmd = std::process::Command::try_from(gix_command::prepare("").with_context(ctx))?;
        assert_eq!(
            format!("{cmd:?}"),
            winfix(format!(r#"GIT_NO_REPLACE_OBJECTS="{expected}" """#))
        );
    }
    Ok(())
}

#[test]
fn ref_namespace_sets_env_only() -> gix_testtools::TestResult {
    let ctx = Context {
        ref_namespace: Some("namespace".into()),
        ..Default::default()
    };
    let cmd = std::process::Command::try_from(gix_command::prepare("").with_context(ctx))?;
    assert_eq!(format!("{cmd:?}"), winfix(r#"GIT_NAMESPACE="namespace" """#));
    Ok(())
}

#[test]
fn literal_pathspecs_sets_env_only() -> gix_testtools::TestResult {
    for value in [false, true] {
        let expected = usize::from(value);
        let ctx = Context {
            literal_pathspecs: Some(value),
            ..Default::default()
        };
        let cmd = std::process::Command::try_from(gix_command::prepare("").with_context(ctx))?;
        assert_eq!(
            format!("{cmd:?}"),
            winfix(format!(r#"GIT_LITERAL_PATHSPECS="{expected}" """#))
        );
    }
    Ok(())
}

#[test]
fn glob_pathspecs_sets_env_only() -> gix_testtools::TestResult {
    for (value, expected) in [
        (false, r#"GIT_NOGLOB_PATHSPECS="1""#),
        (true, r#"GIT_GLOB_PATHSPECS="1""#),
    ] {
        let ctx = Context {
            glob_pathspecs: Some(value),
            ..Default::default()
        };
        let cmd = std::process::Command::try_from(gix_command::prepare("").with_context(ctx))?;
        assert_eq!(format!("{cmd:?}"), winfix(format!(r#"{expected} """#)));
    }
    Ok(())
}

#[test]
fn icase_pathspecs_sets_env_only() -> gix_testtools::TestResult {
    for value in [false, true] {
        let expected = usize::from(value);
        let ctx = Context {
            icase_pathspecs: Some(value),
            ..Default::default()
        };
        let cmd = std::process::Command::try_from(gix_command::prepare("").with_context(ctx))?;
        assert_eq!(
            format!("{cmd:?}"),
            winfix(format!(r#"GIT_ICASE_PATHSPECS="{expected}" """#))
        );
    }
    Ok(())
}
