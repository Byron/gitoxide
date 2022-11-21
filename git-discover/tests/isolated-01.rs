use git_discover::upwards::Options;
use std::path::Path;

#[test]
fn upwards_with_relative_directories_and_optional_ceiling() -> git_testtools::Result {
    let repo = git_testtools::scripted_fixture_repo_read_only("make_basic_repo.sh")?;

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
        assert_eq!(
            repo_path
                .into_repository_and_work_tree_directories()
                .1
                .expect("work dir"),
            Path::new(".."),
        );

        let (repo_path, _trust) =
            git_discover::upwards_opts(search_dir, Default::default()).expect("without ceiling dir we see the same");
        assert_eq!(
            repo_path
                .into_repository_and_work_tree_directories()
                .1
                .expect("work dir"),
            Path::new(".."),
        );
    }

    Ok(())
}
