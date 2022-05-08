use std::path::PathBuf;

use git_discover::repository::Kind;

#[test]
fn from_bare_git_dir() -> crate::Result {
    let dir = repo_path()?.join("bare.git");
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.as_ref(), dir, "the bare .git dir is directly returned");
    assert_eq!(path.kind(), Kind::Bare);
    #[cfg(windows)]
    {
        if is_ci::cached() {
            assert_eq!(trust, git_sec::Trust::Reduced);
        } else {
            assert_eq!(trust, git_sec::Trust::Full);
        }
    }
    #[cfg(not(windows))]
    assert_eq!(trust, git_sec::Trust::Full);
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
    assert_eq!(trust, git_sec::Trust::Full);
    Ok(())
}

#[test]
fn from_git_dir() -> crate::Result {
    let dir = repo_path()?.join(".git");
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.kind(), Kind::WorkTree);
    assert_eq!(
        path.into_repository_and_work_tree_directories().0,
        dir,
        "the .git dir is directly returned if valid"
    );
    assert_eq!(trust, git_sec::Trust::Full);
    Ok(())
}

#[test]
fn from_working_dir() -> crate::Result {
    let dir = repo_path()?;
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.as_ref(), dir, "a working tree dir yields the git dir");
    assert_eq!(path.kind(), Kind::WorkTree);
    assert_eq!(trust, git_sec::Trust::Full);
    Ok(())
}

#[test]
fn from_nested_dir() -> crate::Result {
    let working_dir = repo_path()?;
    let dir = working_dir.join("some/very/deeply/nested/subdir");
    let (path, trust) = git_discover::upwards(&dir)?;
    assert_eq!(path.kind(), Kind::WorkTree);
    assert_eq!(path.as_ref(), working_dir, "a working tree dir yields the git dir");
    assert_eq!(trust, git_sec::Trust::Full);
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
    assert_eq!(path.kind(), Kind::WorkTree);
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
    assert_eq!(path.kind(), Kind::WorkTree);
    assert_eq!(path.as_ref(), working_dir, "we find .git directories on the way");
    assert_eq!(trust, git_sec::Trust::Full);
    Ok(())
}

fn repo_path() -> crate::Result<PathBuf> {
    git_testtools::scripted_fixture_repo_read_only("make_basic_repo.sh")
}
