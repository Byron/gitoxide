mod close {

    use git_lock::acquire::Fail;
    use std::io::Write;

    #[test]
    fn acquire_close_commit_to_existing_file() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let resource = dir.path().join("resource-existing.ext");
        std::fs::write(&resource, b"old state")?;
        let resource_lock = resource.with_extension("ext.lock");
        let mut file = git_lock::File::acquire_to_update_resource(&resource, Fail::Immediately, None)?;
        assert!(resource_lock.is_file());
        file.with_mut(|out| out.write_all(b"hello world"))?;
        let mark = file.close()?;
        assert_eq!(mark.lock_path(), resource_lock);
        assert_eq!(mark.resource_path(), resource);
        assert_eq!(mark.commit()?, resource, "returned and initial resource path match");
        assert_eq!(
            std::fs::read(resource)?,
            &b"hello world"[..],
            "it created the resource and wrote the data"
        );
        assert!(!resource_lock.is_file());
        Ok(())
    }
}

mod commit {
    use git_lock::acquire::Fail;

    #[test]
    fn failure_to_commit_does_return_a_registered_marker() {
        let dir = tempfile::tempdir().unwrap();
        let resource = dir.path().join("resource-existing.ext");
        std::fs::create_dir(&resource).unwrap();
        let mark = git_lock::Marker::acquire_to_hold_resource(&resource, Fail::Immediately, None).unwrap();
        let lock_path = mark.lock_path().to_owned();
        assert!(lock_path.is_file(), "the lock is placed");

        let err = mark
            .commit()
            .expect_err("cannot commit onto existing directory, empty or not");
        assert!(err.instance.lock_path().is_file(), "the lock is still present");

        drop(err);
        assert!(
            !lock_path.is_file(),
            "the lock file is still owned by the lock instance (and ideally still registered, but hard to test)"
        );
    }

    #[test]
    fn failure_to_commit_does_return_a_registered_file() {
        let dir = tempfile::tempdir().unwrap();
        let resource = dir.path().join("resource-existing.ext");
        std::fs::create_dir(&resource).unwrap();
        let file = git_lock::File::acquire_to_update_resource(&resource, Fail::Immediately, None).unwrap();
        let lock_path = file.lock_path().to_owned();
        assert!(lock_path.is_file(), "the lock is placed");

        let err = file
            .commit()
            .expect_err("cannot commit onto existing directory, empty or not");
        assert!(err.instance.lock_path().is_file(), "the lock is still present");
        std::fs::remove_dir(resource).unwrap();
        let (resource, open_file) = err.instance.commit().unwrap();
        let mut open_file = open_file.expect("file to be present as no interrupt has messed with us");

        assert!(
            !lock_path.is_file(),
            "the lock was moved into place, now it's the resource"
        );

        use std::io::Write;
        write!(open_file, "hello").unwrap();
        drop(open_file);
        assert_eq!(
            std::fs::read(resource).unwrap(),
            b"hello".to_vec(),
            "and committing returned a writable file handle"
        );
    }
}

mod acquire {
    use git_lock::acquire;
    use std::io::{ErrorKind, Write};

    fn fail_immediately() -> git_lock::acquire::Fail {
        acquire::Fail::Immediately
    }

    #[test]
    fn lock_create_dir_write_commit() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let resource = dir.path().join("a").join("resource-nonexisting");
        let resource_lock = resource.with_extension("lock");
        let mut file =
            git_lock::File::acquire_to_update_resource(&resource, fail_immediately(), Some(dir.path().into()))?;
        assert_eq!(file.lock_path(), resource_lock);
        assert_eq!(file.resource_path(), resource);
        assert!(resource_lock.is_file());
        file.with_mut(|out| out.write_all(b"hello world"))?;
        assert_eq!(file.commit()?.0, resource, "returned and computed resource path match");
        assert_eq!(
            std::fs::read(resource)?,
            &b"hello world"[..],
            "it created the resource and wrote the data"
        );
        assert!(!resource_lock.is_file());
        Ok(())
    }

    #[test]
    fn lock_write_drop() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let resource = dir.path().join("resource-nonexisting.ext");
        {
            let mut file = git_lock::File::acquire_to_update_resource(&resource, fail_immediately(), None)?;
            file.with_mut(|out| out.write_all(b"probably we will be interrupted"))?;
        }
        assert!(!resource.is_file(), "the file wasn't created");
        Ok(())
    }

    #[test]
    fn lock_non_existing_dir_fails() -> crate::Result {
        let dir = tempfile::tempdir()?;
        let resource = dir.path().join("a").join("resource.ext");
        let res = git_lock::File::acquire_to_update_resource(&resource, fail_immediately(), None);
        assert!(matches!(res, Err(acquire::Error::Io(err)) if err.kind() == ErrorKind::NotFound));
        assert!(dir.path().is_dir(), "it won't meddle with the containing directory");
        assert!(!resource.is_file(), "the resource is not created");
        assert!(
            !resource.parent().unwrap().is_dir(),
            "parent dire wasn't created either"
        );
        Ok(())
    }
}
