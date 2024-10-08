pub fn submodule_repo(name: &str) -> crate::Result<gix::Repository> {
    use crate::util::named_subrepo_opts;
    Ok(named_subrepo_opts(
        "make_submodules.sh",
        name,
        gix::open::Options::isolated(),
    )?)
}

pub fn repo(name: &str) -> crate::Result<gix::Repository> {
    use crate::util::named_subrepo_opts;
    Ok(named_subrepo_opts(
        "make_status_repos.sh",
        name,
        gix::open::Options::isolated(),
    )?)
}

mod index_worktree {
    mod iter {
        use crate::status::{repo, submodule_repo};
        use gix::status::index_worktree::iter::Item;
        use pretty_assertions::assert_eq;

        #[test]
        fn item_size() {
            assert_eq!(
                std::mem::size_of::<gix::status::index_worktree::iter::Item>(),
                264,
                "The size is pretty huge and goes down ideally"
            );
        }

        #[test]
        fn submodule_modification() -> crate::Result {
            let repo = submodule_repo("modified-untracked-and-submodule-head-changed-and-modified")?;
            let mut status = repo
                .status(gix::progress::Discard)?
                .index_worktree_options_mut(|opts| {
                    opts.sorting =
                        Some(gix::status::plumbing::index_as_worktree_with_renames::Sorting::ByPathCaseSensitive);
                })
                .into_index_worktree_iter(Vec::new())?;
            let items: Vec<_> = status.by_ref().filter_map(Result::ok).collect();
            assert_eq!(items.len(), 3, "1 untracked, 1 modified file, 1 submodule modification");
            Ok(())
        }

        #[test]
        fn untracked_files_collapse_by_default() -> crate::Result {
            let repo = repo("untracked-only")?;
            let status = repo
                .status(gix::progress::Discard)?
                .index_worktree_options_mut(|opts| {
                    opts.sorting =
                        Some(gix::status::plumbing::index_as_worktree_with_renames::Sorting::ByPathCaseSensitive);
                })
                .into_index_worktree_iter(Vec::new())?;
            let items: Vec<_> = status.filter_map(Result::ok).collect();
            assert_eq!(
                items,
                [
                    Item::DirectoryContents {
                        entry: gix_dir::Entry {
                            rela_path: "new".into(),
                            status: gix_dir::entry::Status::Untracked,
                            property: None,
                            disk_kind: Some(gix_dir::entry::Kind::Directory),
                            index_kind: None,
                            pathspec_match: Some(gix_dir::entry::PathspecMatch::Always),
                        },
                        collapsed_directory_status: None
                    },
                    Item::DirectoryContents {
                        entry: gix_dir::Entry {
                            rela_path: "subdir/untracked".into(),
                            status: gix_dir::entry::Status::Untracked,
                            property: None,
                            disk_kind: Some(gix_dir::entry::Kind::File),
                            index_kind: None,
                            pathspec_match: Some(gix_dir::entry::PathspecMatch::Always),
                        },
                        collapsed_directory_status: None
                    }
                ],
                "'new/untracked' gets collapsed, but the second untracked is in a folder with a tracked file.\
                This collapsing behaviour is the default."
            );
            Ok(())
        }

        #[test]
        fn untracked_files_settings_none() -> crate::Result {
            let mut repo = repo("untracked-only")?;
            repo.config_snapshot_mut()
                .set_value(&gix::config::tree::Status::SHOW_UNTRACKED_FILES, "no")?;

            let mut status = repo
                .status(gix::progress::Discard)?
                .index_worktree_options_mut(|opts| {
                    opts.sorting =
                        Some(gix::status::plumbing::index_as_worktree_with_renames::Sorting::ByPathCaseSensitive);
                })
                .into_index_worktree_iter(Vec::new())?;
            let items: Vec<_> = status.by_ref().filter_map(Result::ok).collect();
            assert_eq!(items, [], "no untracked files are found…");
            assert_eq!(
                status.outcome_mut().expect("iteration done").index_worktree.dirwalk,
                None,
                "…as there was no directory walk"
            );
            Ok(())
        }

        #[test]
        fn early_drop_for_is_dirty_emulation() -> crate::Result {
            let repo = submodule_repo("modified-untracked-and-submodule-head-changed-and-modified")?;
            let is_dirty = repo
                .status(gix::progress::Discard)?
                .index_worktree_submodules(gix::status::Submodule::AsConfigured { check_dirty: true })
                .index_worktree_options_mut(|opts| {
                    opts.sorting =
                        Some(gix::status::plumbing::index_as_worktree_with_renames::Sorting::ByPathCaseSensitive);
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
    use crate::status::submodule_repo;

    #[test]
    fn various_changes_positive() -> crate::Result {
        let repo = submodule_repo("modified-untracked-and-submodule-head-changed-and-modified")?;
        assert!(repo.is_dirty()?, "The repository has various changes");
        Ok(())
    }

    #[test]
    fn submodule_changes_are_picked_up() -> crate::Result {
        let repo = submodule_repo("submodule-head-changed")?;
        assert!(repo.is_dirty()?, "head-changes are also discovered");
        Ok(())
    }

    #[test]
    fn untracked_files_are_excluded() -> crate::Result {
        let repo = submodule_repo("module1")?;
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
        let repo = submodule_repo("with-submodules")?;
        assert!(!repo.is_dirty()?, "there are no changes");
        Ok(())
    }
}
