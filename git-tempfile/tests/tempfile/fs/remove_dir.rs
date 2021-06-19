mod empty_until_boundary {
    use git_tempfile::remove_dir;
    use std::{io, path::Path};

    #[test]
    fn boundary_must_contain_target_dir() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let (target, boundary) = (dir.path().join("a"), dir.path().join("b"));
        std::fs::create_dir(&target)?;
        std::fs::create_dir(&boundary)?;
        assert!(matches!(remove_dir::empty_until_boundary(&target, &boundary),
                            Err(err) if err.kind() == io::ErrorKind::InvalidInput));
        Ok(())
    }
    #[test]
    fn boundary_being_the_target_dir_always_succeeds_and_we_do_nothing() -> crate::Result {
        let dir = tempfile::tempdir()?;
        assert_eq!(remove_dir::empty_until_boundary(dir.path(), dir.path())?, dir.path());
        assert!(dir.path().is_dir(), "it won't touch the boundary");
        Ok(())
    }
    #[test]
    fn a_directory_which_doesnt_exist_to_start_with_is_ok() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let target = dir.path().join("does-not-exist");
        assert_eq!(remove_dir::empty_until_boundary(&target, dir.path())?, target);
        Ok(())
    }
    #[test]
    fn boundary_directory_doesnt_have_to_exist_either_if_the_target_doesnt() -> crate::Result {
        let boundary = Path::new("/boundary");
        let target = Path::new("/boundary/target");
        assert_eq!(remove_dir::empty_until_boundary(target, boundary)?, target);
        Ok(())
    }
    #[test]
    fn nested_directory_deletion_works() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let nested = dir.path().join("a").join("b").join("to-delete");
        std::fs::create_dir_all(&nested)?;
        assert_eq!(remove_dir::empty_until_boundary(&nested, dir.path())?, nested);
        assert!(!nested.is_dir(), "it actually deleted the nested directory");
        Ok(())
    }
}

mod iter {}
