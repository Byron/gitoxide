mod reflog {
    use crate::file;

    #[test]
    fn iter() -> crate::Result {
        let store = file::store()?;
        let r = store.find_one_existing("HEAD")?;
        let mut buf = Vec::new();
        assert_eq!(r.log_iter(&mut buf)?.expect("log exists").count(), 1);
        Ok(())
    }
}

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
        Store::at("base doesnt matter", Default::default())
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
                    assert_eq!(reference.target().as_name(), $ref);
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
