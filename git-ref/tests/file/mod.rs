use git_ref::file;

fn store() -> crate::Result<file::Store> {
    let path = git_testtools::scripted_fixture_repo_read_only("make_ref_repository.sh")?;
    Ok(file::Store::from(path.join(".git")))
}

mod store {
    mod find_one {
        use crate::file::store;
        use std::path::Path;

        mod existing {
            use crate::file::store;
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
                            Err(git_ref::file::find_one::existing::Error::NotFound(name)) => {
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
                ("origin/main", "refs/remotes/origin/main", git_ref::Kind::Peeled),
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
            for (partial_name, reason, is_err) in &[
                ("foobar", "does not exist", false),
                ("broken", "does not parse", true),
                ("../escaping", "an invalid ref name", true),
            ] {
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

mod log {
    mod iter {
        mod forward {
            use bstr::B;
            use git_hash::ObjectId;
            use std::path::PathBuf;

            fn reflog_dir() -> crate::Result<PathBuf> {
                Ok(
                    git_testtools::scripted_fixture_repo_read_only("make_repo_for_reflog.sh")?
                        .join(".git")
                        .join("logs"),
                )
            }
            fn reflog(name: &str) -> crate::Result<Vec<u8>> {
                Ok(std::fs::read(reflog_dir()?.join(name))?)
            }

            #[test]
            fn all_success() -> crate::Result {
                let log = reflog("HEAD")?;
                let iter = git_ref::file::log::iter::forward(&log);
                assert_eq!(iter.count(), 5, "the log as a known amount of entries");

                let mut iter = git_ref::file::log::iter::forward(&log);
                let line = iter.next().unwrap()?;
                assert_eq!(line.previous_oid(), ObjectId::null_sha1());
                assert_eq!(line.new_oid, B("134385f6d781b7e97062102c6a483440bfda2a03"));
                assert_eq!(line.message, B("commit (initial): c1"));
                assert!(iter.all(|l| l.is_ok()), "all lines parse fine");
                Ok(())
            }

            #[test]
            fn a_single_failure_does_not_abort_iteration() {
                let log_first_broken = "0000000000000000000000000000000000000000 134385fbroken7062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit
0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c1\n";

                let mut iter = git_ref::file::log::iter::forward(log_first_broken.as_bytes());
                let err = iter.next().expect("error is not none").expect_err("the line is broken");
                assert_eq!(err.to_string(), "In line 1: Parse error:\nTakeWhileMN at: 134385fbroken7062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000\tcommit\n\nin section '<new-hexsha>', at: 134385fbroken7062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000\tcommit\n\nin section '<old-hexsha> <new-hexsha> <name> <<email>> <timestamp> <tz>\\t<message>', at: 0000000000000000000000000000000000000000 134385fbroken7062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000\tcommit\n\n");
                assert!(iter.next().expect("a second line").is_ok(), "line parses ok");
                assert!(iter.next().is_none(), "iterator exhausted");
            }
        }
    }
}

mod reference {
    mod peel {
        use crate::file;
        use git_testtools::hex_to_id;
        use std::path::Path;

        #[test]
        fn one_level() -> crate::Result {
            let store = file::store()?;
            let r = store.find_one_existing("HEAD")?;
            assert_eq!(r.kind(), git_ref::Kind::Symbolic, "there is something to peel");

            let nr = r.peel_one_level().expect("exists").expect("no failure");
            assert!(
                matches!(nr.target(), git_ref::Target::Peeled(_)),
                "iteration peels a single level"
            );
            assert!(nr.peel_one_level().is_none(), "end of iteration");
            assert_eq!(
                nr.target(),
                git_ref::Target::Peeled(&hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03")),
                "we still have the peeled target"
            );
            Ok(())
        }

        #[test]
        fn to_id_multi_hop() -> crate::Result {
            let store = file::store()?;
            let mut r = store.find_one_existing("multi-link")?;
            assert_eq!(r.kind(), git_ref::Kind::Symbolic, "there is something to peel");

            assert_eq!(
                r.peel_to_id_in_place()?,
                hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03")
            );
            assert_eq!(r.relative_path, Path::new("refs/remotes/origin/multi-link-target3"));

            Ok(())
        }

        #[test]
        fn to_id_cycle() -> crate::Result {
            let store = file::store()?;
            let mut r = store.find_one_existing("loop-a")?;
            assert_eq!(r.kind(), git_ref::Kind::Symbolic, "there is something to peel");
            assert_eq!(r.relative_path, Path::new("refs/loop-a"));

            assert!(matches!(
                r.peel_to_id_in_place().unwrap_err(),
                git_ref::file::reference::peel::to_id::Error::Cycle { .. }
            ));
            assert_eq!(
                r.relative_path,
                Path::new("refs/loop-a"),
                "the ref is not changed on error"
            );
            Ok(())
        }
    }

    mod parse {
        use git_ref::file::Store;

        fn store() -> Store {
            Store::at("base doesnt matter")
        }

        mod invalid {
            use crate::file::reference::parse::store;
            use git_ref::file::Reference;

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
            use crate::file::reference::parse::store;
            use bstr::ByteSlice;
            use git_ref::file::Reference;
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
