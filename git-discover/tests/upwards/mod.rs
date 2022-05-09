use std::path::PathBuf;

use git_discover::repository::Kind;

fn expected_trust() -> git_sec::Trust {
    #[cfg(not(windows))]
    {
        git_sec::Trust::Full
    }
    #[cfg(windows)]
    {
        if is_ci::cached() {
            git_sec::Trust::Reduced
        } else {
            git_sec::Trust::Full
        }
    }
}

#[test]
fn from_bare_git_dir() -> crate::Result {
    let dir = repo_path()?.join("bare.git");
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.as_ref(), dir, "the bare .git dir is directly returned");
    assert_eq!(path.kind(), Kind::Bare);
    assert_eq!(trust, expected_trust());
    Ok(())
}

#[test]
fn from_inside_bare_git_dir() -> crate::Result {
    let git_dir = repo_path()?.join("bare.git");
    let dir = git_dir.join("objects");
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(
        path.as_ref(),
        git_dir,
        "the bare .git dir is found while traversing upwards"
    );
    assert_eq!(path.kind(), Kind::Bare);
    assert_eq!(trust, expected_trust());
    Ok(())
}

#[test]
fn from_git_dir() -> crate::Result {
    let dir = repo_path()?.join(".git");
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.kind(), Kind::WorkTree { linked_git_dir: None });
    assert_eq!(
        path.into_repository_and_work_tree_directories().0,
        dir,
        "the .git dir is directly returned if valid"
    );
    assert_eq!(trust, expected_trust());
    Ok(())
}

#[test]
fn from_working_dir() -> crate::Result {
    let dir = repo_path()?;
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.as_ref(), dir, "a working tree dir yields the git dir");
    assert_eq!(path.kind(), Kind::WorkTree { linked_git_dir: None });
    assert_eq!(trust, expected_trust());
    Ok(())
}

#[test]
fn from_nested_dir() -> crate::Result {
    let working_dir = repo_path()?;
    let dir = working_dir.join("some/very/deeply/nested/subdir");
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.kind(), Kind::WorkTree { linked_git_dir: None });
    assert_eq!(path.as_ref(), working_dir, "a working tree dir yields the git dir");
    assert_eq!(trust, expected_trust());
    Ok(())
}

#[test]
fn from_dir_with_dot_dot() -> crate::Result {
    // This would be neater if we could just change the actual working directory,
    // but Rust tests run in parallel by default so we'd interfere with other tests.
    // Instead ensure it finds the gitoxide repo instead of a test repo if we crawl
    // up far enough. (This tests that `discover::existing` canonicalizes paths before
    // exploring ancestors.)
    let working_dir = repo_path()?;
    let dir = working_dir.join("some/very/deeply/nested/subdir/../../../../../..");
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.kind(), Kind::WorkTree { linked_git_dir: None });
    // On CI on windows we get a cursor like this with a question mark so our prefix check won't work.
    // We recover, but that means this assertion will fail.
    // &cursor = "\\\\?\\D:\\a\\gitoxide\\gitoxide\\.git"
    // &cwd = "D:\\a\\gitoxide\\gitoxide\\git-repository"
    #[cfg(not(windows))]
    assert_eq!(
        path.as_ref(),
        std::path::Path::new(".."),
        "there is only the minimal amount of relative path components to see this worktree"
    );
    assert_ne!(
        path.as_ref().canonicalize()?,
        working_dir.canonicalize()?,
        "a relative path that climbs above the test repo should yield the gitoxide repo"
    );
    assert_eq!(trust, git_sec::Trust::Full);
    Ok(())
}

#[test]
fn from_nested_dir_inside_a_git_dir() -> crate::Result {
    let working_dir = repo_path()?;
    let dir = working_dir.join(".git").join("objects");
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.kind(), Kind::WorkTree { linked_git_dir: None });
    assert_eq!(path.as_ref(), working_dir, "we find .git directories on the way");
    assert_eq!(trust, expected_trust());
    Ok(())
}

#[test]
fn from_existing_worktree() {
    let top_level_repo = repo_path().unwrap();
    for (discover_path, expected_worktree_path, expected_git_dir) in [
        (top_level_repo.join("worktrees/a"), "worktrees/a", ".git/worktrees/a"),
        (
            top_level_repo.join("worktrees/from-bare/c"),
            "worktrees/from-bare/c",
            "bare.git/worktrees/c",
        ),
    ] {
        let (path, trust) = git_discover::upwards(discover_path).unwrap();
        assert!(matches!(path, git_discover::repository::Path::LinkedWorkTree { .. }));

        assert_eq!(trust, expected_trust());
        let (git_dir, worktree) = path.into_repository_and_work_tree_directories();
        #[cfg(not(windows))]
        assert_eq!(
            git_dir.strip_prefix(top_level_repo.canonicalize().unwrap()),
            Ok(std::path::Path::new(expected_git_dir)),
            "we don't skip over worktrees and discover their git dir (gitdir is absolute in file)"
        );
        #[cfg(windows)]
        assert_eq!(
            git_dir.canonicalize().unwrap(),
            top_level_repo.join(expected_git_dir).canonicalize().unwrap(),
            "we don't skip over worktrees and discover their git dir (gitdir is absolute in file)"
        );
        let worktree = worktree.expect("linked worktree is set");
        assert_eq!(
            worktree.strip_prefix(&top_level_repo),
            Ok(std::path::Path::new(expected_worktree_path)),
            "the worktree path is the .git file's directory"
        );

        assert!(
            git_discover::is_git(&git_dir).is_err(),
            "we aren't able to detect git directories from private worktrees and that's by design"
        );
    }
}

fn repo_path() -> crate::Result<PathBuf> {
    git_testtools::scripted_fixture_repo_read_only("make_basic_repo.sh")
}
