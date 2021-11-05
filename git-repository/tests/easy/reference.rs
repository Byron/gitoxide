mod find {
    use std::convert::{TryFrom, TryInto};

    use git_ref as refs;
    use git_repository::prelude::ReferenceAccessExt;
    use git_testtools::hex_to_id;

    fn repo() -> crate::Result<git_repository::Easy> {
        crate::repo("make_references_repo.sh").map(Into::into)
    }

    /// Gain an understanding how uses might want to call this function, and see what happens
    #[test]
    fn possible_inputs() -> crate::Result {
        let repo = repo()?;
        repo.find_reference("dt1")?;
        repo.find_reference(&String::from("dt1"))?; // Owned Strings don't have an impl for PartialName

        struct CustomType(String);
        impl<'a> TryFrom<&'a CustomType> for refs::PartialNameRef<'a> {
            type Error = refs::name::Error;

            fn try_from(value: &'a CustomType) -> Result<Self, Self::Error> {
                refs::PartialNameRef::try_from(&value.0)
            }
        }
        repo.find_reference(&CustomType("dt1".into()))?;

        struct CustomName {
            remote: &'static str,
            branch: &'static str,
        }

        impl CustomName {
            fn to_partial_name(&self) -> String {
                format!("{}/{}", self.remote, self.branch)
            }
            fn to_partial_name_from_string(&self) -> git_ref::PartialNameRef<'static> {
                self.to_partial_name().try_into().expect("cannot fail")
            }
            fn to_partial_name_from_bstring(&self) -> git_ref::PartialNameRef<'static> {
                git_object::bstr::BString::from(self.to_partial_name())
                    .try_into()
                    .expect("cannot fail")
            }
            fn to_full_name(&self) -> git_ref::FullName {
                format!("{}/{}", self.remote, self.branch)
                    .try_into()
                    .expect("always valid")
            }
        }

        impl<'a> TryFrom<&'a CustomName> for refs::PartialNameRef<'static> {
            type Error = refs::name::Error;

            fn try_from(value: &'a CustomName) -> Result<Self, Self::Error> {
                refs::PartialNameRef::try_from(value.to_partial_name())
            }
        }

        let name = CustomName {
            remote: "origin",
            branch: "main",
        };
        repo.find_reference(&name.to_partial_name())?;
        repo.find_reference(name.to_partial_name())?;
        repo.find_reference(name.to_partial_name_from_string())?;
        repo.find_reference(name.to_partial_name_from_bstring())?;
        repo.find_reference(name.to_full_name().to_partial())?;
        repo.find_reference(&name)?;

        Ok(())
    }

    #[test]
    fn and_peel() -> crate::Result {
        let repo = repo()?;
        let mut packed_tag_ref = repo.try_find_reference("dt1")?.expect("tag to exist");
        assert_eq!(packed_tag_ref.name(), "refs/tags/dt1".try_into()?);

        assert_eq!(
            packed_tag_ref.inner.target,
            refs::Target::Peeled(hex_to_id("4c3f4cce493d7beb45012e478021b5f65295e5a3")),
            "it points to a tag object"
        );

        let object = packed_tag_ref.peel_to_id_in_place()?;
        let the_commit = hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03");
        assert_eq!(object, the_commit, "it is assumed to be fully peeled");
        assert_eq!(
            object,
            packed_tag_ref.peel_to_id_in_place()?,
            "peeling again yields the same object"
        );

        let mut symbolic_ref = repo.find_reference("multi-link-target1")?;
        assert_eq!(symbolic_ref.name(), "refs/heads/multi-link-target1".try_into()?);
        assert_eq!(symbolic_ref.peel_to_id_in_place()?, the_commit);
        assert_eq!(
            symbolic_ref.name(),
            "refs/remotes/origin/multi-link-target3".try_into()?,
            "it follows symbolic refs, too"
        );
        assert_eq!(symbolic_ref.into_fully_peeled_id()?, the_commit, "idempotency");
        Ok(())
    }
}
