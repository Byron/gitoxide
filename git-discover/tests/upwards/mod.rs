use std::path::PathBuf;

use git_discover::repository::Kind;

fn expected_trust() -> git_sec::Trust {
    git_sec::Trust::Full
}

mod ceiling_dirs;

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
fn from_non_existing_worktree() {
    let top_level_repo = repo_path().unwrap();
    let (path, _trust) = git_discover::upwards(top_level_repo.join("worktrees/b-private-dir-deleted")).unwrap();
    assert_eq!(path, git_discover::repository::Path::WorkTree(top_level_repo.clone()));

    let (path, _trust) =
        git_discover::upwards(top_level_repo.join("worktrees/from-bare/d-private-dir-deleted")).unwrap();
    assert_eq!(path, git_discover::repository::Path::WorkTree(top_level_repo));
}

#[test]
fn from_existing_worktree_inside_dot_git() {
    let top_level_repo = repo_path().unwrap();
    let (path, _trust) = git_discover::upwards(top_level_repo.join(".git/worktrees/a")).unwrap();
    let suffix = std::path::Path::new(top_level_repo.file_name().unwrap())
        .join("worktrees")
        .join("a");
    assert!(
        matches!(path, git_discover::repository::Path::LinkedWorkTree { work_dir, .. } if work_dir.ends_with(suffix)),
        "we can handle to start from within a (somewhat partial) worktree git dir"
    );
}

#[test]
fn from_non_existing_worktree_inside_dot_git() {
    let top_level_repo = repo_path().unwrap();
    let (path, _trust) = git_discover::upwards(top_level_repo.join(".git/worktrees/c-worktree-deleted")).unwrap();
    let suffix = std::path::Path::new(top_level_repo.file_name().unwrap())
        .join("worktrees")
        .join("c-worktree-deleted");
    assert!(
        matches!(path, git_discover::repository::Path::LinkedWorkTree { work_dir, .. } if work_dir.ends_with(suffix)),
        "it's no problem if work-dirs don't exist - this can be discovered later and a lot of operations are possible anyway."
    );
}

#[test]
fn from_existing_worktree() -> crate::Result {
    let top_level_repo = repo_path()?;
    for (discover_path, expected_worktree_path, expected_git_dir) in [
        (top_level_repo.join("worktrees/a"), "worktrees/a", ".git/worktrees/a"),
        (
            top_level_repo.join("worktrees/from-bare/c"),
            "worktrees/from-bare/c",
            "bare.git/worktrees/c",
        ),
    ] {
        let (path, trust) = git_discover::upwards(discover_path)?;
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
            git_dir.canonicalize()?,
            top_level_repo.join(expected_git_dir).canonicalize()?,
            "we don't skip over worktrees and discover their git dir (gitdir is absolute in file)"
        );
        let worktree = worktree.expect("linked worktree is set");
        assert_eq!(
            worktree.strip_prefix(&top_level_repo),
            Ok(std::path::Path::new(expected_worktree_path)),
            "the worktree path is the .git file's directory"
        );
    }
    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn cross_fs() -> crate::Result {
    use git_discover::upwards::Options;
    use std::os::unix::fs::symlink;
    use std::process::Command;

    let top_level_repo = git_testtools::scripted_fixture_repo_writable("make_basic_repo.sh")?;

    let _cleanup = {
        // Create an empty dmg file
        let dmg_location = tempfile::tempdir()?;
        let dmg_file = dmg_location.path().join("temp.dmg");
        Command::new("hdiutil")
            .args(&["create", "-size", "1m"])
            .arg(&dmg_file)
            .status()?;

        // Mount dmg file into temporary location
        let mount_point = tempfile::tempdir()?;
        Command::new("hdiutil")
            .args(&["attach", "-nobrowse", "-mountpoint"])
            .arg(mount_point.path())
            .arg(&dmg_file)
            .status()?;

        // Ensure that the mount point is always cleaned up
        let cleanup = defer::defer({
            let arg = mount_point.path().to_owned();
            move || {
                Command::new("hdiutil")
                    .arg("detach")
                    .arg(arg)
                    .status()
                    .expect("detach temporary test dmg filesystem successfully");
            }
        });

        // Symlink the mount point into the repo
        symlink(mount_point.path(), top_level_repo.path().join("remote"))?;
        cleanup
    };

    let res = git_discover::upwards(top_level_repo.path().join("remote"))
        .expect_err("the cross-fs option should prevent us from discovering the repo");
    assert!(matches!(
        res,
        git_discover::upwards::Error::NoGitRepositoryWithinFs { .. }
    ));

    let (repo_path, _trust) = git_discover::upwards_opts(
        &top_level_repo.path().join("remote"),
        Options {
            cross_fs: true,
            ..Default::default()
        },
    )
    .expect("the cross-fs option should allow us to discover the repo");

    assert_eq!(
        repo_path
            .into_repository_and_work_tree_directories()
            .1
            .expect("work dir")
            .file_name(),
        top_level_repo.path().file_name()
    );

    Ok(())
}

fn repo_path() -> crate::Result<PathBuf> {
    git_testtools::scripted_fixture_repo_read_only("make_basic_repo.sh")
}
