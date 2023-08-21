use crate::util::named_subrepo_opts;

#[test]
fn bare_repo_with_index() -> crate::Result {
    let repo = named_subrepo_opts(
        "make_basic_repo.sh",
        "bare-repo-with-index.git",
        gix::open::Options::isolated(),
    )?;
    assert!(
        repo.is_bare(),
        "it's properly classified as it reads the configuration (and has no worktree)"
    );
    Ok(())
}

#[test]
fn none_bare_repo_without_index() -> crate::Result {
    let repo = named_subrepo_opts(
        "make_basic_repo.sh",
        "non-bare-repo-without-index",
        gix::open::Options::isolated(),
    )?;
    assert!(!repo.is_bare(), "worktree isn't dependent on an index file");
    assert!(repo.worktree().is_some());
    Ok(())
}

mod missing_config_file {

    use crate::util::named_subrepo_opts;

    #[test]
    fn bare() -> crate::Result {
        let repo = named_subrepo_opts("make_config_repos.sh", "bare-no-config", gix::open::Options::isolated())?;
        assert!(
            repo.is_bare(),
            "without config, we can't really know what the repo is actually but can guess by not having a worktree"
        );
        assert_eq!(repo.work_dir(), None);
        assert!(repo.worktree().is_none());
        assert_eq!(
            repo.config_snapshot().meta().source,
            gix::config::Source::Local,
            "config always refers to the local one for safety"
        );
        Ok(())
    }

    #[test]
    fn non_bare() -> crate::Result {
        let repo = named_subrepo_opts(
            "make_config_repos.sh",
            "worktree-no-config",
            gix::open::Options::isolated(),
        )?;
        assert!(repo.work_dir().is_some());
        assert!(repo.worktree().is_some());
        assert!(
            !repo.is_bare(),
            "without config, we can't really know what the repo is actually but can guess as there is a worktree"
        );
        assert_eq!(
            repo.config_snapshot().meta().source,
            gix::config::Source::Local,
            "config always refers to the local one for safety"
        );
        Ok(())
    }
}

mod not_a_repository {

    #[test]
    fn shows_proper_error() -> crate::Result {
        for name in ["empty-dir", "with-files"] {
            let name = format!("not-a-repo-{name}");
            let repo_path = gix_testtools::scripted_fixture_read_only("make_config_repos.sh")?.join(name);
            let err = gix::open_opts(&repo_path, gix::open::Options::isolated()).unwrap_err();
            assert!(matches!(err, gix::open::Error::NotARepository { path, .. } if path == repo_path));
        }
        Ok(())
    }
}

mod open_path_as_is {

    use crate::util::{named_subrepo_opts, repo_opts};

    fn open_path_as_is() -> gix::open::Options {
        gix::open::Options::isolated().open_path_as_is(true)
    }

    #[test]
    fn bare_repos_open_normally() -> crate::Result {
        assert!(named_subrepo_opts("make_basic_repo.sh", "bare.git", open_path_as_is())?.is_bare());
        Ok(())
    }

    #[test]
    fn worktrees_cannot_be_opened() -> crate::Result {
        let err = repo_opts("make_basic_repo.sh", open_path_as_is()).unwrap_err();
        assert!(matches!(err, gix::open::Error::NotARepository { .. }));
        Ok(())
    }

    #[test]
    fn git_dir_within_worktrees_open_normally() -> crate::Result {
        assert!(!named_subrepo_opts("make_basic_repo.sh", ".git", open_path_as_is())?.is_bare());
        Ok(())
    }
}

mod submodules {
    use std::path::Path;

    #[test]
    fn by_their_worktree_checkout_and_git_modules_dir() {
        let dir = gix_testtools::scripted_fixture_read_only("make_submodules.sh").unwrap();
        let parent_repo = Path::new("with-submodules");
        let modules = parent_repo.join(".git").join("modules");
        for module in ["m1", "dir/m1"] {
            let submodule_m1_workdir = parent_repo.join(module);
            let submodule_m1_gitdir = modules.join(module);

            for discover_dir in [
                submodule_m1_workdir.clone(),
                submodule_m1_workdir.join("subdir"),
                submodule_m1_gitdir.clone(),
            ] {
                let repo = discover_repo(discover_dir).unwrap();
                // assert_eq!(repo.kind(), gix::Kind::Submodule);
                assert_eq!(repo.work_dir().expect("non-bare"), dir.join(&submodule_m1_workdir));
                assert_eq!(repo.git_dir(), dir.join(&submodule_m1_gitdir));

                let repo = gix::open_opts(repo.work_dir().expect("non-bare"), gix::open::Options::isolated()).unwrap();
                assert_eq!(repo.kind(), gix::repository::Kind::Submodule);
                assert_eq!(repo.work_dir().expect("non-bare"), dir.join(&submodule_m1_workdir));
                assert_eq!(repo.git_dir(), dir.join(&submodule_m1_gitdir));
            }
        }
    }

    fn discover_repo(name: impl AsRef<Path>) -> crate::Result<gix::Repository> {
        let dir = gix_testtools::scripted_fixture_read_only("make_submodules.sh")?;
        let repo_dir = dir.join(name);
        Ok(gix::ThreadSafeRepository::discover_opts(
            repo_dir,
            Default::default(),
            gix_sec::trust::Mapping {
                full: crate::restricted(),
                reduced: crate::restricted(),
            },
        )?
        .to_thread_local())
    }
}

mod object_caches {

    use crate::util::named_subrepo_opts;

    #[test]
    fn default_git_and_custom_caches() -> crate::Result {
        let opts = gix::open::Options::isolated();
        let repo = named_subrepo_opts("make_config_repos.sh", "object-caches", opts)?;
        assert_eq!(repo.objects.has_object_cache(), cfg!(feature = "comfort"));
        assert_eq!(repo.objects.has_pack_cache(), cfg!(feature = "comfort"));
        Ok(())
    }

    #[test]
    fn disabled() -> crate::Result {
        let opts = gix::open::Options::isolated();
        let repo = named_subrepo_opts("make_config_repos.sh", "disabled-object-caches", opts)?;
        assert!(!repo.objects.has_object_cache());
        assert!(!repo.objects.has_pack_cache());
        Ok(())
    }
}

mod worktree {
    use gix::open;

    #[test]
    fn with_worktree_configs() -> gix_testtools::Result {
        let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
        let fixture_dir = gix_testtools::scripted_fixture_read_only("make_worktree_repo_with_configs.sh")?;
        let worktree_base = manifest_dir.join(&fixture_dir).join("repo/.git/worktrees");

        {
            let base = open(fixture_dir.join("repo"))?;
            let base_config = base.config_snapshot();

            assert_eq!(
                base.work_dir(),
                Some(fixture_dir.join("repo").as_path()),
                "the main worktree"
            );
            assert_eq!(base.git_dir(), fixture_dir.join("repo/.git"), "git dir and…");
            assert_eq!(
                base.common_dir(),
                fixture_dir.join("repo/.git"),
                "…common dir are the same"
            );

            assert_eq!(
                base_config.string("worktree.setting").expect("exists").as_ref(),
                "set in the main worktree"
            );
            assert_eq!(
                base_config.string("shared.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
            assert_eq!(
                base_config.string("override.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
        }

        {
            let wt1 = open(fixture_dir.join("wt-1"))?;
            let wt1_config = wt1.config_snapshot();
            assert_eq!(
                wt1.work_dir(),
                Some(fixture_dir.join("wt-1").as_path()),
                "a linked worktree in its own location"
            );
            assert_eq!(
                wt1.git_dir(),
                worktree_base.join("wt-1"),
                "whose git-dir is within the common dir"
            );
            assert_eq!(
                wt1.common_dir(),
                worktree_base.join("wt-1/../.."),
                "the common dir is the `git-dir` of the repository with the main worktree"
            );

            assert_eq!(
                wt1_config.string("worktree.setting").expect("exists").as_ref(),
                "set in wt-1"
            );
            assert_eq!(
                wt1_config.string("shared.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
            assert_eq!(
                wt1_config.string("override.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
        }

        {
            let wt2 = open(fixture_dir.join("wt-2"))?;
            let wt2_config = wt2.config_snapshot();
            assert_eq!(
                wt2.work_dir(),
                Some(fixture_dir.join("wt-2").as_path()),
                "another linked worktree as sibling to wt-1"
            );
            assert_eq!(wt2.git_dir(), worktree_base.join("wt-2"));
            assert_eq!(wt2.common_dir(), worktree_base.join("wt-2/../.."));

            assert_eq!(
                wt2_config.string("worktree.setting").expect("exists").as_ref(),
                "set in wt-2"
            );
            assert_eq!(
                wt2_config.string("shared.setting").expect("exists").as_ref(),
                "set in the shared config"
            );
            assert_eq!(
                wt2_config.string("override.setting").expect("exists").as_ref(),
                "override in wt-2"
            );
        }

        Ok(())
    }
}
