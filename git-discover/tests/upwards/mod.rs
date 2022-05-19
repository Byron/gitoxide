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

mod ceiling_dirs {
    use crate::upwards::repo_path;
    use git_discover::upwards::Options;
    use std::path::Path;
    #[cfg(windows)]
    use std::path::PathBuf;

    fn assert_repo_is_current_workdir(path: git_discover::repository::Path, work_dir: &Path) {
        assert_eq!(
            path.into_repository_and_work_tree_directories()
                .1
                .expect("work dir")
                .file_name(),
            work_dir.file_name()
        );
    }

    #[test]
    fn single() -> crate::Result {
        let work_dir = repo_path()?;
        let base_dir = work_dir.canonicalize()?;
        let dir = base_dir.join("some/very/deeply/nested/subdir");
        let (repo_path, _trust) = git_discover::upwards_opts(
            &dir,
            Options {
                ceiling_dirs: &[base_dir.clone()],
                ..Default::default()
            },
        )
        .expect("ceiling dir should allow us to discover the repo");
        assert_repo_is_current_workdir(repo_path, &work_dir);

        let err = git_discover::upwards_opts(
            &dir,
            Options {
                ceiling_dirs: &[base_dir.join("some")],
                ..Default::default()
            },
        )
        .expect_err("ceiling dir prevents discovery as it ends on level too early");
        assert!(matches!(
            err,
            git_discover::upwards::Error::NoGitRepositoryWithinCeiling { ceiling_height: 5, .. }
        ));

        Ok(())
    }

    #[test]
    fn multiple() -> crate::Result {
        let work_dir = repo_path()?;
        let base_dir = work_dir.canonicalize()?;
        let dir = base_dir.join("some/very/deeply/nested/subdir");
        let (repo_path, _trust) = git_discover::upwards_opts(
            &dir,
            Options {
                ceiling_dirs: &[
                    base_dir.clone(),
                    base_dir.join("some/very/deeply/nested/subdir/too-deep"),
                    base_dir.join("some/very/deeply/nested/unrelated-dir"),
                    base_dir.join("a/completely/unrelated/dir"),
                ],
                ..Default::default()
            },
        )
        .expect("ceiling dir should allow us to discover the repo");
        assert_repo_is_current_workdir(repo_path, &work_dir);

        let err = git_discover::upwards_opts(
            &dir,
            Options {
                ceiling_dirs: &[work_dir, base_dir.join("some")],
                ..Default::default()
            },
        )
        .expect_err("more restrictive ceiling dirs overrule less restrictive ones");
        assert!(matches!(
            err,
            git_discover::upwards::Error::NoGitRepositoryWithinCeiling { ceiling_height: 5, .. }
        ));

        Ok(())
    }

    // these are special because all of our base paths are relative unless canonicalized.
    #[test]
    fn special_relative() -> crate::Result {
        let work_dir = repo_path()?;
        let base_dir = work_dir.canonicalize()?;
        let dir = base_dir.join("some/very/deeply/nested/subdir");
        let (repo_path, _trust) = git_discover::upwards_opts(
            &dir,
            Options {
                ceiling_dirs: &[Path::new("./some").into(), Path::new("").into()],
                ..Default::default()
            },
        )
        .expect("the repo can be discovered because the relative ceiling has nothing to do with the repo location");
        assert_repo_is_current_workdir(repo_path, &work_dir);

        Ok(())
    }

    #[test]
    fn special_relative_base() -> crate::Result {
        let work_dir = repo_path()?;
        let base_dir = work_dir.canonicalize()?;
        let dir = base_dir.join("some/very/deeply/nested/subdir/../../../../../..");
        let (repo_path, _trust) = git_discover::upwards_opts(
            &dir,
            Options {
                ceiling_dirs: &[base_dir.clone()],
                ..Default::default()
            },
        )
        .expect("the repo can be discovered because the relative ceiling has nothing to do with the repo location");

        assert_ne!(
            repo_path.as_ref().canonicalize()?,
            base_dir,
            "a relative path that climbs above the test repo should yield the gitoxide repo"
        );

        Ok(())
    }

    #[test]
    fn no_root_base() -> crate::Result {
        let work_dir = repo_path()?;
        let base_dir = work_dir.canonicalize()?;
        let dir = work_dir.join("some");
        let res = git_discover::upwards_opts(
            &dir,
            Options {
                ceiling_dirs: &[base_dir],
                ..Default::default()
            },
        );

        assert!(matches!(
            res,
            Err(git_discover::upwards::Error::DirectoryNotAbsolute { .. })
        ));

        Ok(())
    }

    #[cfg(windows)]
    fn strip_prefix(path: PathBuf) -> PathBuf {
        path.to_str()
            .expect("path needs to be valid unicode to strip verbatim paths prefixes")
            .strip_prefix(r"\\?\")
            .map(PathBuf::from)
            .unwrap_or(path)
    }

    #[test]
    #[cfg(windows)]
    #[should_panic]
    fn verbatim_prefix_win() {
        let work_dir = repo_path().expect("repo path to be created successfully");
        let base_dir = work_dir.canonicalize().expect("repo path to exist");
        let repo_path_no_prefix = strip_prefix(base_dir.clone());

        let dir = base_dir.join(r"some");
        git_discover::upwards_opts(
            &dir,
            Options {
                ceiling_dirs: &[repo_path_no_prefix.join(r"some")],
                ..Default::default()
            },
        )
        .expect_err("ceiling dir prevents discovery even if the ceiling is not a verbatim path");
    }
}

fn repo_path() -> crate::Result<PathBuf> {
    git_testtools::scripted_fixture_repo_read_only("make_basic_repo.sh")
}
