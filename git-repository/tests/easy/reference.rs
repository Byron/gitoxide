mod find {
    use std::convert::TryInto;

    use git_ref as refs;
    use git_testtools::hex_to_id;

    fn repo() -> crate::Result<git_repository::easy::Handle> {
        crate::repo("make_references_repo.sh").map(Into::into)
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
