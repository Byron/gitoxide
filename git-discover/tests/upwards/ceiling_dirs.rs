use std::path::Path;

use git_discover::upwards::Options;

use crate::upwards::repo_path;

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
fn git_dir_candidate_within_ceiling_allows_discovery() -> crate::Result {
    let work_dir = repo_path()?;
    let dir = work_dir.join("some/very/deeply/nested/subdir");
    let (repo_path, _trust) = git_discover::upwards_opts(
        dir,
        Options {
            ceiling_dirs: vec![work_dir.clone()],
            ..Default::default()
        },
    )
    .expect("ceiling dir should allow us to discover the repo");
    assert_repo_is_current_workdir(repo_path, &work_dir);

    Ok(())
}

#[test]
fn ceiling_dir_limits_are_respected_and_prevent_discovery() -> crate::Result {
    let work_dir = repo_path()?;
    let dir = work_dir.join("some/very/deeply/nested/subdir");

    let err = git_discover::upwards_opts(
        dir,
        Options {
            ceiling_dirs: vec![work_dir.join("some/../some")],
            ..Default::default()
        },
    )
    .expect_err("ceiling dir prevents discovery as it ends on level too early, and they are also absolutized");
    assert!(matches!(
        err,
        git_discover::upwards::Error::NoGitRepositoryWithinCeiling { ceiling_height: 5, .. }
    ));

    Ok(())
}

#[test]
fn no_matching_ceiling_dir_error_can_be_suppressed() -> crate::Result {
    let work_dir = repo_path()?;
    let dir = work_dir.join("some/very/deeply/nested/subdir");
    let (repo_path, _trust) = git_discover::upwards_opts(
        dir,
        Options {
            match_ceiling_dir_or_error: false,
            ceiling_dirs: vec![
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

    Ok(())
}

#[test]
fn more_restrictive_ceiling_dirs_overrule_less_restrictive_ones() -> crate::Result {
    let work_dir = repo_path()?;
    let dir = work_dir.join("some/very/deeply/nested/subdir");
    let err = git_discover::upwards_opts(
        dir,
        Options {
            ceiling_dirs: vec![work_dir.clone(), work_dir.join("some")],
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

#[test]
fn ceiling_dirs_are_not_processed_differently_than_the_git_dir_candidate() -> crate::Result {
    let work_dir = repo_path()?;
    let dir = work_dir.join("some/very/deeply/nested/subdir/../../../../../..");
    let (repo_path, _trust) = git_discover::upwards_opts(
        dir,
        Options {
            match_ceiling_dir_or_error: false,
            ceiling_dirs: vec![Path::new("./some").into()],
            ..Default::default()
        },
    )
    .expect("the repo can be discovered because the relative ceiling doesn't _look_ like it has something to do with the git dir candidate");

    assert_ne!(
        &repo_path.as_ref().canonicalize()?,
        &work_dir,
        "a relative path that climbs above the test repo should yield the gitoxide repo"
    );

    Ok(())
}

#[test]
fn no_matching_ceiling_dirs_errors_by_default() -> crate::Result {
    let relative_work_dir = repo_path()?;
    let dir = relative_work_dir.join("some");
    let res = git_discover::upwards_opts(
        dir,
        Options {
            ceiling_dirs: vec!["/something/somewhere".into()],
            ..Default::default()
        },
    );

    assert!(
        matches!(res, Err(git_discover::upwards::Error::NoMatchingCeilingDir)),
        "the canonicalized ceiling dir doesn't have the same root as the git dir candidate, and can never match."
    );
    Ok(())
}

#[test]
fn ceilings_are_adjusted_to_match_search_dir() -> crate::Result {
    let relative_work_dir = repo_path()?;
    let cwd = std::env::current_dir()?;
    let absolute_ceiling_dir = git_path::realpath_opts(&relative_work_dir, &cwd, 8)?;
    let dir = relative_work_dir.join("some");
    assert!(dir.is_relative());
    let (repo_path, _trust) = git_discover::upwards_opts(
        &dir,
        Options {
            ceiling_dirs: vec![absolute_ceiling_dir],
            ..Default::default()
        },
    )?;
    assert_repo_is_current_workdir(repo_path, &relative_work_dir);

    assert!(relative_work_dir.is_relative());
    let absolute_dir = git_path::realpath_opts(relative_work_dir.join("some"), cwd, 8)?;
    let (repo_path, _trust) = git_discover::upwards_opts(
        absolute_dir,
        Options {
            ceiling_dirs: vec![relative_work_dir.clone()],
            ..Default::default()
        },
    )?;
    assert_repo_is_current_workdir(repo_path, &relative_work_dir);
    Ok(())
}
