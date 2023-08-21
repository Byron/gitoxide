mod modules_file {
    use crate::submodule::repo;

    #[test]
    fn none_if_not_present() -> crate::Result {
        let repo = repo("module1")?;
        assert!(repo.open_modules_file()?.is_none(), "it's OK to not have such a file");
        assert!(
            repo.modules()?.is_none(),
            "this is reflected in the persisted version as well"
        );
        Ok(())
    }

    #[test]
    fn is_read_from_worktree() -> crate::Result {
        let repo = repo("with-submodules")?;
        let modules = repo.modules()?.expect("present");
        assert_eq!(
            modules.names().collect::<Vec<_>>(),
            &["m1", "dir/m1"],
            "dir/m1 is listed only in the worktree version"
        );
        Ok(())
    }

    #[test]
    fn is_read_from_index_if_not_in_worktree() -> crate::Result {
        let repo = repo("with-submodules-in-index")?;
        assert!(
            repo.open_modules_file()?.is_none(),
            ".gitmodules not available in worktree"
        );
        let modules = repo.modules()?.expect("present as loaded from index");
        assert_eq!(
            modules.names().collect::<Vec<_>>(),
            &["m1", "dir/m1"],
            "dir/m1 is listed only in the index version"
        );
        Ok(())
    }

    #[test]
    fn is_read_from_tree_if_not_in_index() -> crate::Result {
        let repo = repo("with-submodules-in-tree")?;
        assert!(
            repo.open_modules_file()?.is_none(),
            ".gitmodules not available in worktree"
        );
        let modules = repo.modules()?.expect("present as loaded from tree");
        assert_eq!(
            modules.names().collect::<Vec<_>>(),
            &["m1"],
            "only m1 has been committed and thus is available in the tree at HEAD"
        );
        Ok(())
    }
}

mod submodules {
    use gix::bstr::BString;

    use crate::{submodule::repo, util::hex_to_id};

    #[test]
    fn all_modules_are_active_by_default() -> crate::Result {
        let repo = repo("with-submodules")?;
        let id = hex_to_id("e046f3e51d955840619fc7d01fbd9a469663de22");
        assert_eq!(
            repo.submodules()?
                .expect("submodules")
                .map(|sm| (
                    sm.name().to_owned(),
                    sm.path().expect("valid path").into_owned(),
                    sm.head_id().expect("valid"),
                    sm.index_id().expect("valid"),
                    sm.is_active().expect("no config error")
                ))
                .collect::<Vec<_>>(),
            [
                ("m1", "m1", Some(id), Some(id), true),
                ("dir/m1", "dir/m1", None, Some(id), true)
            ]
            .into_iter()
            .map(|(name, path, head_id, index_id, is_active)| (
                BString::from(name),
                BString::from(path),
                head_id,
                index_id,
                is_active
            ))
            .collect::<Vec<_>>()
        );

        Ok(())
    }
}
