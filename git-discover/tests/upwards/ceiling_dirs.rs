use crate::upwards::repo_path;
use git_discover::upwards::Options;
use std::path::Path;

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
    let dir = work_dir.join("some/very/deeply/nested/subdir");
    let (repo_path, _trust) = git_discover::upwards_opts(
        &dir,
        Options {
            ceiling_dirs: &vec![work_dir.clone()],
            ..Default::default()
        },
    )
    .expect("ceiling dir should allow us to discover the repo");
    assert_repo_is_current_workdir(repo_path, &work_dir);

    let err = git_discover::upwards_opts(
        &dir,
        Options {
            ceiling_dirs: &vec![work_dir.join("some")],
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
    let dir = work_dir.join("some/very/deeply/nested/subdir");
    let (repo_path, _trust) = git_discover::upwards_opts(
        &dir,
        Options {
            match_ceiling_dir_or_error: false,
            ceiling_dirs: &vec![
                work_dir.canonicalize()?,
                work_dir.join("some/very/deeply/nested/subdir/too-deep"),
                work_dir.join("some/very/deeply/nested/unrelated-dir"),
                work_dir.join("a/completely/unrelated/dir"),
            ],
            ..Default::default()
        },
    )
    .expect("ceiling dir should allow us to discover the repo");
    assert_repo_is_current_workdir(repo_path, &work_dir);

    let err = git_discover::upwards_opts(
        &dir,
        Options {
            ceiling_dirs: &vec![work_dir.clone(), work_dir.join("some")],
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
    let dir = work_dir.join("some/very/deeply/nested/subdir/../../../../../..");
    let (repo_path, _trust) = git_discover::upwards_opts(
        &dir,
        Options {
            match_ceiling_dir_or_error: false,
            ceiling_dirs: &vec![Path::new("./some").into()],
            ..Default::default()
        },
    )
    .expect("the repo can be discovered because the relative ceiling has nothing to do with the repo location");

    assert_ne!(
        &repo_path.as_ref().canonicalize()?,
        &work_dir,
        "a relative path that climbs above the test repo should yield the gitoxide repo"
    );

    Ok(())
}

#[test]
fn no_root_base() -> crate::Result {
    let relative_work_dir = repo_path()?;
    let absolute_ceiling_dir = relative_work_dir.canonicalize()?;
    let dir = relative_work_dir.join("some");
    let res = git_discover::upwards_opts(
        &dir,
        Options {
            ceiling_dirs: &[absolute_ceiling_dir],
            ..Default::default()
        },
    );

    assert!(matches!(res, Err(git_discover::upwards::Error::NoMatchingCeilingDir)));
    Ok(())
}

#[test]
#[cfg(windows)]
fn verbatim_prefix_win() {
    #[cfg(windows)]
    fn strip_prefix(path: std::path::PathBuf) -> std::path::PathBuf {
        path.to_str()
            .expect("path needs to be valid unicode to strip verbatim paths prefixes")
            .strip_prefix(r"\\?\")
            .map(ToOwned::to_owned)
            .unwrap_or(path)
    }

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
