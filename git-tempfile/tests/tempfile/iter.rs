mod create_dir {
    use git_tempfile::{
        create_dir,
        create_dir::{Error::*, Retries},
    };
    pub use std::io::ErrorKind::*;

    #[test]
    fn an_existing_directory_causes_immediate_success() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let mut it = create_dir::Iter::new(dir.path());
        assert_eq!(
            it.next().expect("item").expect("success"),
            dir.path(),
            "first iteration is immediately successful"
        );
        assert!(it.next().is_none(), "iterator exhausted afterwards");
        Ok(())
    }

    #[test]
    fn a_single_directory_can_be_created_too() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let new_dir = dir.path().join("new");
        let mut it = create_dir::Iter::new(&new_dir);
        assert_eq!(
            it.next().expect("item").expect("success"),
            &new_dir,
            "first iteration is immediately successful"
        );
        assert!(it.next().is_none(), "iterator exhausted afterwards");
        assert!(new_dir.is_dir(), "the directory exists");
        Ok(())
    }

    #[test]
    fn multiple_intermediate_directories_are_created_automaticaly() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let new_dir = dir.path().join("s1").join("s2").join("new");
        let mut it = create_dir::Iter::new(&new_dir);
        assert!(
            matches!(it.next(), Some(Err(Intermediate(k))) if k == NotFound),
            "dir is not present"
        );
        assert!(
            matches!(it.next(), Some(Err(Intermediate(k))) if k == NotFound),
            "parent dir is not present"
        );
        assert_eq!(
            it.next().expect("item").expect("success"),
            new_dir.parent().unwrap().parent().unwrap(),
            "first subdir is created"
        );
        assert_eq!(
            it.next().expect("item").expect("success"),
            new_dir.parent().unwrap(),
            "second subdir is created"
        );
        assert_eq!(
            it.next().expect("item").expect("success"),
            new_dir,
            "target directory is created"
        );
        assert!(it.next().is_none(), "iterator depleted");
        assert!(new_dir.is_dir(), "the directory exists");
        Ok(())
    }

    #[test]
    fn multiple_intermediate_directories_are_created_up_to_retries_limit() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let new_dir = dir.path().join("s1").join("s2").join("new");
        let mut it = create_dir::Iter::new_with_retries(
            &new_dir,
            Retries {
                on_create_directory: 1,
                ..Default::default()
            },
        );
        assert!(
            matches!(it.next(), Some(Err(Permanent{ attempts, dir, err })) if attempts == 1
                                                                    && err.kind() == NotFound
                                                                    && dir == new_dir),
            "parent dir is not present and we run out of attempts"
        );
        assert!(it.next().is_none(), "iterator depleted");
        assert!(!new_dir.is_dir(), "the wasn't created");
        Ok(())
    }
}
