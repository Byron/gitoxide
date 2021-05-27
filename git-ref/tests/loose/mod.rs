mod store {
    mod find_one {
        use git_ref::loose;
        use std::path::Path;

        fn store() -> crate::Result<loose::Store> {
            let path = git_testtools::scripted_fixture_repo_read_only("make_ref_repository.sh")?;
            Ok(loose::Store::from(path.join(".git")))
        }

        mod existing {
            use crate::loose::store::find_one::store;
            use std::path::Path;

            #[test]
            fn success_and_failure() -> crate::Result {
                let store = store()?;
                for (partial_name, expected_path) in &[("main", Some("refs/heads/main")), ("does-not-exist", None)] {
                    let reference = store.find_one_existing(*partial_name);
                    match expected_path {
                        Some(expected_path) => assert_eq!(reference?.relative_path, Path::new(expected_path)),
                        None => match reference {
                            Ok(_) => panic!("Expected error"),
                            Err(git_ref::loose::find::existing::Error::NotFound(name)) => {
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
                ("origin", "refs/remotes/origin/HEAD", git_ref::Kind::Symbolic),
                ("origin/main", "refs/remotes/origin/main", git_ref::Kind::Peeled),
                ("HEAD", "HEAD", git_ref::Kind::Symbolic),
                ("t1", "refs/tags/t1", git_ref::Kind::Peeled),
                ("main", "refs/heads/main", git_ref::Kind::Peeled),
                ("heads/main", "refs/heads/main", git_ref::Kind::Peeled),
                ("refs/heads/main", "refs/heads/main", git_ref::Kind::Peeled),
            ] {
                let reference = store.find_one(*partial_name)?.expect("exists");
                assert_eq!(reference.relative_path, Path::new(expected_path));
                assert_eq!(reference.target().kind(), *expected_ref_kind);
            }
            Ok(())
        }

        #[test]
        fn failure() -> crate::Result {
            let store = store()?;
            for (partial_name, reason, is_err) in
                &[("foobar", "does not exist", false), ("broken", "does not parse", true)]
            {
                let reference = store.find_one(*partial_name);
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
}

mod reference {
    mod parse {
        use git_ref::loose::Store;

        fn store() -> Store {
            Store::new("base doesnt matter")
        }

        mod invalid {
            use crate::loose::reference::parse::store;
            use git_ref::loose::Reference;

            macro_rules! mktest {
                ($name:ident, $input:literal, $err:literal) => {
                    #[test]
                    fn $name() {
                        let store = store();
                        let err = Reference::try_from_path(&store, "name", $input).unwrap_err();
                        assert_eq!(err.to_string(), $err);
                    }
                };
            }

            mktest!(hex_id, b"foobar", "\"foobar\" could not be parsed");
            mktest!(ref_tag, b"reff: hello", "\"reff: hello\" could not be parsed");
        }
        mod valid {
            use crate::loose::reference::parse::store;
            use bstr::ByteSlice;
            use git_ref::loose::Reference;
            use git_testtools::hex_to_id;

            macro_rules! mktest {
                ($name:ident, $input:literal, $kind:path, $id:expr, $ref:expr) => {
                    #[test]
                    fn $name() {
                        let store = store();
                        let reference = Reference::try_from_path(&store, "name", $input).unwrap();
                        assert_eq!(reference.kind(), $kind);
                        assert_eq!(reference.target().as_id(), $id);
                        assert_eq!(reference.target().as_ref(), $ref);
                    }
                };
            }

            mktest!(
                peeled,
                b"c5241b835b93af497cda80ce0dceb8f49800df1c\n",
                git_ref::Kind::Peeled,
                Some(hex_to_id("c5241b835b93af497cda80ce0dceb8f49800df1c").as_ref()),
                None
            );

            mktest!(
                symbolic,
                b"ref: refs/heads/main\n",
                git_ref::Kind::Symbolic,
                None,
                Some(b"refs/heads/main".as_bstr())
            );

            mktest!(
                symbolic_more_than_one_space,
                b"ref:        refs/foobar\n",
                git_ref::Kind::Symbolic,
                None,
                Some(b"refs/foobar".as_bstr())
            );
        }
    }
}
