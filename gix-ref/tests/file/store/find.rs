mod existing {
    use std::convert::{TryFrom, TryInto};

    use gix_ref::{PartialName, PartialNameRef};

    use crate::{file::store_at, hex_to_id};

    #[test]
    fn with_packed_refs() -> crate::Result {
        let store = store_at("make_packed_ref_repository_for_overlay.sh")?;
        let c1 = hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03");
        let r = store.find("main")?;
        assert_eq!(r.target.into_id(), c1);
        assert_eq!(r.name.as_bstr(), "refs/heads/main");
        Ok(())
    }

    // TODO: figure this out
    #[test]
    fn possible_inputs() -> crate::Result {
        let store = crate::file::store()?;
        store.find_loose("dt1")?;
        store.find_loose(&String::from("dt1"))?; // Owned Strings don't have an impl for PartialName

        struct CustomType(String);
        impl<'a> TryFrom<&'a CustomType> for &'a PartialNameRef {
            type Error = gix_ref::name::Error;

            fn try_from(value: &'a CustomType) -> Result<Self, Self::Error> {
                value.0.as_str().try_into()
            }
        }
        store.find_loose(&CustomType("dt1".into()))?;

        struct CustomName {
            remote: &'static str,
            branch: &'static str,
        }

        impl CustomName {
            fn to_partial_name(&self) -> String {
                format!("{}/{}", self.remote, self.branch)
            }
            fn to_partial_name_from_string(&self) -> PartialName {
                self.to_partial_name().try_into().expect("cannot fail")
            }
            fn to_partial_name_from_bstring(&self) -> PartialName {
                gix_object::bstr::BString::from(self.to_partial_name())
                    .try_into()
                    .expect("cannot fail")
            }
            fn to_full_name(&self) -> gix_ref::FullName {
                format!("{}/{}", self.remote, self.branch)
                    .try_into()
                    .expect("always valid")
            }
        }

        impl<'a> TryFrom<&'a CustomName> for PartialName {
            type Error = gix_ref::name::Error;

            fn try_from(value: &'a CustomName) -> Result<Self, Self::Error> {
                PartialName::try_from(value.to_partial_name())
            }
        }

        let name = CustomName {
            remote: "origin",
            branch: "main",
        };
        store.find_loose(&name.to_partial_name())?;
        // TODO: this effectively needs a `Cow<'_, PartialNameRef>`, but we are not allowed to implement conversions for it.
        //       After having been there, I don't want to have a `PartialNameCow(Cow<'_, PartialNameRef)` anymore, nor
        //       copies of `TryFrom/TryInto` traits in our crate.
        //       Make it work once we can implement standard traits for Cow<OurType>.
        // store.find_loose(&name)?;
        // store.find_loose(name.to_partial_name())?;
        store.find_loose(&name.to_partial_name_from_string())?;
        store.find_loose(&name.to_partial_name_from_bstring())?;
        store.find_loose(&name.to_full_name())?;
        store.find_loose(name.to_full_name().as_ref())?;
        store.find_loose(name.to_full_name().as_ref().as_partial_name())?;
        store.find_loose(&PartialName::try_from(name.remote)?.join(name.branch.into())?)?;
        store.find_loose(&PartialName::try_from("origin")?.join("main".into())?)?;
        store.find_loose(&PartialName::try_from("origin")?.join(String::from("main").as_str().into())?)?;
        store.find_loose(&PartialName::try_from("origin")?.join("main".into())?)?;

        Ok(())
    }
}

mod loose {
    use crate::{file::store, hex_to_id};

    mod existing {
        use std::path::Path;

        use crate::file::store;

        #[test]
        fn success_and_failure() -> crate::Result {
            let store = store()?;
            for (partial_name, expected_path) in &[("main", Some("refs/heads/main")), ("does-not-exist", None)] {
                let reference = store.find_loose(*partial_name);
                match expected_path {
                    Some(expected_path) => assert_eq!(reference?.name.as_bstr(), expected_path),
                    None => match reference {
                        Ok(_) => panic!("Expected error"),
                        Err(gix_ref::file::find::existing::Error::NotFound { name }) => {
                            assert_eq!(name, Path::new(*partial_name));
                        }
                        Err(err) => panic!("Unexpected err: {err:?}"),
                    },
                }
            }
            Ok(())
        }
    }

    #[test]
    fn fetch_head_can_be_parsed() -> crate::Result {
        let store = store()?;
        assert_eq!(
            store.find_loose("FETCH_HEAD")?.target.id(),
            hex_to_id("9064ea31fae4dc59a56bdd3a06c0ddc990ee689e"),
            "despite being special, we are able to read the first commit out of a typical FETCH_HEAD"
        );
        Ok(())
    }

    #[test]
    fn success() -> crate::Result {
        let store = store()?;
        for (partial_name, expected_path, expected_ref_kind) in &[
            ("dt1", "refs/tags/dt1", gix_ref::Kind::Peeled),     // tags before heads
            ("FETCH_HEAD", "FETCH_HEAD", gix_ref::Kind::Peeled), // special ref
            ("heads/dt1", "refs/heads/dt1", gix_ref::Kind::Peeled),
            ("d1", "refs/d1", gix_ref::Kind::Peeled), // direct refs before heads
            ("heads/d1", "refs/heads/d1", gix_ref::Kind::Peeled),
            ("HEAD", "HEAD", gix_ref::Kind::Symbolic), // it finds shortest paths first
            ("origin", "refs/remotes/origin/HEAD", gix_ref::Kind::Symbolic),
            ("origin/HEAD", "refs/remotes/origin/HEAD", gix_ref::Kind::Symbolic),
            ("origin/main", "refs/remotes/origin/main", gix_ref::Kind::Peeled),
            ("t1", "refs/tags/t1", gix_ref::Kind::Peeled),
            ("main", "refs/heads/main", gix_ref::Kind::Peeled),
            ("heads/main", "refs/heads/main", gix_ref::Kind::Peeled),
            ("refs/heads/main", "refs/heads/main", gix_ref::Kind::Peeled),
        ] {
            let reference = store.try_find_loose(*partial_name)?.expect("exists");
            assert_eq!(reference.name.as_bstr(), expected_path);
            assert_eq!(reference.target.to_ref().kind(), *expected_ref_kind);
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
            let reference = store.try_find_loose(*partial_name);
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
