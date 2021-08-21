mod existing {
    use git_testtools::hex_to_id;

    use crate::file::store_at;

    #[test]
    fn with_packed_refs() -> crate::Result {
        let store = store_at("make_packed_ref_repository_for_overlay.sh")?;
        let c1 = hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03");
        let packed = store.packed_buffer()?;
        let r = store.find_existing("main", packed.as_ref())?;
        assert_eq!(r.target().borrow().as_id().expect("peeled"), c1);
        assert_eq!(r.name().as_bstr(), "refs/heads/main");
        Ok(())
    }
}

mod loose {
    use crate::file::store;

    mod existing {
        use std::path::Path;

        use crate::file::store;

        #[test]
        fn success_and_failure() -> crate::Result {
            let store = store()?;
            for (partial_name, expected_path) in &[("main", Some("refs/heads/main")), ("does-not-exist", None)] {
                let reference = store.loose_find_existing(*partial_name);
                match expected_path {
                    Some(expected_path) => assert_eq!(reference?.name.as_bstr(), expected_path),
                    None => match reference {
                        Ok(_) => panic!("Expected error"),
                        Err(git_ref::file::find::existing::Error::NotFound(name)) => {
                            assert_eq!(name, Path::new(*partial_name));
                        }
                        Err(err) => panic!("Unexpected err: {:?}", err),
                    },
                }
            }
            Ok(())
        }
    }

    #[test]
    fn success() -> crate::Result {
        let store = store()?;
        for (partial_name, expected_path, expected_ref_kind) in &[
            ("dt1", "refs/tags/dt1", git_ref::Kind::Peeled), // tags before heads
            ("heads/dt1", "refs/heads/dt1", git_ref::Kind::Peeled),
            ("d1", "refs/d1", git_ref::Kind::Peeled), // direct refs before heads
            ("heads/d1", "refs/heads/d1", git_ref::Kind::Peeled),
            ("HEAD", "HEAD", git_ref::Kind::Symbolic), // it finds shortest paths first
            ("origin", "refs/remotes/origin/HEAD", git_ref::Kind::Symbolic),
            ("origin/HEAD", "refs/remotes/origin/HEAD", git_ref::Kind::Symbolic),
            ("origin/main", "refs/remotes/origin/main", git_ref::Kind::Peeled),
            ("t1", "refs/tags/t1", git_ref::Kind::Peeled),
            ("main", "refs/heads/main", git_ref::Kind::Peeled),
            ("heads/main", "refs/heads/main", git_ref::Kind::Peeled),
            ("refs/heads/main", "refs/heads/main", git_ref::Kind::Peeled),
        ] {
            let reference = store.loose_find(*partial_name)?.expect("exists");
            assert_eq!(reference.name.as_bstr(), expected_path);
            assert_eq!(reference.target.borrow().kind(), *expected_ref_kind);
        }
        Ok(())
    }

    #[test]
    fn failure() -> crate::Result {
        let store = store()?;
        for (partial_name, reason, is_err) in &[
            ("foobar", "does not exist", false),
            ("broken", "does not parse", true),
            ("../escaping", "an invalid ref name", true),
        ] {
            let reference = store.loose_find(*partial_name);
            if *is_err {
                assert!(reference.is_err(), "{}", reason);
            } else {
                let reference = reference?;
                assert!(reference.is_none(), "{}", reason);
            }
        }
        Ok(())
    }
}
