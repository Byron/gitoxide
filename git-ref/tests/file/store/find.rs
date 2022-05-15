mod existing {
    use git_testtools::hex_to_id;

    use crate::file::store_at;

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
    // Gain an understanding how uses might want to call this function, and see what happens
    // #[test]
    // fn possible_inputs() -> crate::Result {
    //     let store = crate::file::store()?;
    //     store.find_loose("dt1")?;
    //     store.find_loose(&String::from("dt1"))?; // Owned Strings don't have an impl for PartialName
    //
    //     struct CustomType(String);
    //     impl<'a> git_ref::name::TryFrom<&'a CustomType> for Cow<'a, PartialNameRef> {
    //         type Error = git_ref::name::Error;
    //
    //         fn try_from(value: &'a CustomType) -> Result<Self, Self::Error> {
    //             Cow::<'_, PartialNameRef>::try_from(&value.0)
    //         }
    //     }
    //     store.find_loose(&CustomType("dt1".into()))?;
    //
    //     struct CustomName {
    //         remote: &'static str,
    //         branch: &'static str,
    //     }
    //
    //     impl CustomName {
    //         fn to_partial_name(&self) -> String {
    //             format!("{}/{}", self.remote, self.branch)
    //         }
    //         fn to_partial_name_from_string(&self) -> PartialNameCow<'static> {
    //             self.to_partial_name().try_into().expect("cannot fail")
    //         }
    //         fn to_partial_name_from_bstring(&self) -> PartialNameCow<'static> {
    //             git_object::bstr::BString::from(self.to_partial_name())
    //                 .try_into()
    //                 .expect("cannot fail")
    //         }
    //         fn to_full_name(&self) -> git_ref::FullName {
    //             format!("{}/{}", self.remote, self.branch)
    //                 .try_into()
    //                 .expect("always valid")
    //         }
    //     }
    //
    //     impl<'a> TryFrom<&'a CustomName> for PartialNameCow<'static> {
    //         type Error = git_ref::name::Error;
    //
    //         fn try_from(value: &'a CustomName) -> Result<Self, Self::Error> {
    //             PartialNameCow::try_from(value.to_partial_name())
    //         }
    //     }
    //
    //     let name = CustomName {
    //         remote: "origin",
    //         branch: "main",
    //     };
    //     store.find_loose(&name.to_partial_name())?;
    //     store.find_loose(name.to_partial_name())?;
    //     store.find_loose(name.to_partial_name_from_string())?;
    //     store.find_loose(name.to_partial_name_from_bstring())?;
    //     store.find_loose(name.to_full_name().to_partial())?;
    //     store.find_loose(&name)?;
    //     store.find_loose(PartialNameCow::try_from(name.remote)?.join(name.branch)?)?;
    //     store.find_loose(PartialNameCow::try_from("origin")?.join("main")?)?;
    //     store.find_loose(PartialNameCow::try_from("origin")?.join(String::from("main"))?)?;
    //     store.find_loose(PartialNameCow::try_from("origin")?.join("main")?)?;
    //
    //     Ok(())
    // }
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
                let reference = store.find_loose(*partial_name);
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
