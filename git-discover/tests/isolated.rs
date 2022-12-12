use std::path::{Path, PathBuf};

use git_discover::upwards::Options;
use serial_test::serial;

#[test]
#[serial]
fn upwards_with_relative_directories_and_optional_ceiling() -> git_testtools::Result {
    let repo = git_testtools::scripted_fixture_read_only("make_basic_repo.sh")?;

    std::env::set_current_dir(repo.join("subdir"))?;
    let cwd = std::env::current_dir()?;

    for (search_dir, ceiling_dir_component) in [
        (".", ".."),
        (".", "./.."),
        ("./.", "./.."),
        (".", "./does-not-exist/../.."),
    ] {
        let ceiling_dir = cwd.join(ceiling_dir_component);
        let (repo_path, _trust) = git_discover::upwards_opts(
            search_dir,
            Options {
                ceiling_dirs: vec![ceiling_dir],
                ..Default::default()
            },
        )
        .expect("ceiling dir should allow us to discover the repo");
        assert_repo_is_current_workdir(repo_path, Path::new(".."));

        let (repo_path, _trust) =
            git_discover::upwards_opts(search_dir, Default::default()).expect("without ceiling dir we see the same");
        assert_repo_is_current_workdir(repo_path, Path::new(".."));

        let (repo_path, _trust) = git_discover::upwards_opts(
            search_dir,
            Options {
                ceiling_dirs: vec![PathBuf::from("..")],
                ..Default::default()
            },
        )
        .expect("purely relative ceiling dirs work as well");
        assert_repo_is_current_workdir(repo_path, Path::new(".."));

        let err = git_discover::upwards_opts(
            search_dir,
            Options {
                ceiling_dirs: vec![PathBuf::from(".")],
                ..Default::default()
            },
        )
        .unwrap_err();

        assert!(
            matches!(
                err,
                git_discover::upwards::Error::NoGitRepositoryWithinCeiling { ceiling_height: 1, .. }
            ),
            "limiting the ceiling to the CWD cannot work as it's just an empty dir"
        );
    }

    Ok(())
}

fn assert_repo_is_current_workdir(path: git_discover::repository::Path, work_dir: &Path) {
    assert_eq!(
        path.into_repository_and_work_tree_directories().1.expect("work dir"),
        work_dir,
    );
}
