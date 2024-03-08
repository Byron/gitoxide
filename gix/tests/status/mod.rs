pub fn repo(name: &str) -> crate::Result<gix::Repository> {
    use crate::util::named_subrepo_opts;
    Ok(named_subrepo_opts(
        "make_submodules.sh",
        name,
        gix::open::Options::isolated(),
    )?)
}

mod index_worktree {
    mod iter {
        use crate::status::repo;

        #[test]
        fn submodule_modification() -> crate::Result {
            let repo = repo("modified-untracked-and-submodule-head-changed-and-modified")?;
            let mut status = repo
                .status(gix::progress::Discard)?
                .index_worktree_options_mut(|opts| {
                    opts.sorting =
                        Some(gix::status::plumbing::index_as_worktree_with_renames::Sorting::ByPathCaseSensitive)
                })
                .into_index_worktree_iter(Vec::new())?;
            let items: Vec<_> = status.by_ref().filter_map(Result::ok).collect();
            assert_eq!(items.len(), 3, "1 untracked, 1 modified file, 1 submodule modification");
            Ok(())
        }

        #[test]
        fn early_drop_for_is_dirty_emulation() -> crate::Result {
            let repo = repo("modified-untracked-and-submodule-head-changed-and-modified")?;
            let is_dirty = repo
                .status(gix::progress::Discard)?
                .index_worktree_submodules(gix::status::Submodule::AsConfigured { check_dirty: true })
                .index_worktree_options_mut(|opts| {
                    opts.sorting =
                        Some(gix::status::plumbing::index_as_worktree_with_renames::Sorting::ByPathCaseSensitive)
                })
                .into_index_worktree_iter(Vec::new())?
                .next()
                .is_some();
            assert!(is_dirty, "this should abort the work as quickly as possible");
            Ok(())
        }
    }
}

mod is_dirty {
    use crate::status::repo;

    #[test]
    fn various_changes_positive() -> crate::Result {
        let repo = repo("modified-untracked-and-submodule-head-changed-and-modified")?;
        assert!(repo.is_dirty()?, "The repository has various changes");
        Ok(())
    }

    #[test]
    fn submodule_changes_are_picked_up() -> crate::Result {
        let repo = repo("submodule-head-changed")?;
        assert!(repo.is_dirty()?, "head-changes are also discoverd");
        Ok(())
    }

    #[test]
    fn untracked_files_are_excluded() -> crate::Result {
        let repo = repo("module1")?;
        assert_eq!(
            repo.status(gix::progress::Discard)?
                .into_index_worktree_iter(Vec::new())?
                .count(),
            1,
            "there is one untracked file"
        );
        assert!(
            !repo.is_dirty()?,
            "untracked files aren't taken into consideration, just like `git describe` which ignores them"
        );
        Ok(())
    }

    #[test]
    fn no_changes() -> crate::Result {
        let repo = repo("with-submodules")?;
        assert!(!repo.is_dirty()?, "there are no changes");
        Ok(())
    }
}
