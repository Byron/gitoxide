fn repo() -> crate::Result<git_repository::Easy> {
    crate::repo("make_references_repo.sh").map(Into::into)
}

mod find {
    use std::convert::TryInto;

    use git_ref as refs;
    use git_repository::prelude::*;
    use git_testtools::hex_to_id;

    use crate::reference::repo;

    #[test]
    fn find_and_peel() {
        let repo = repo().unwrap();
        let mut packed_tag_ref = repo.try_find_reference("dt1").unwrap().expect("tag to exist");
        assert_eq!(packed_tag_ref.name(), "refs/tags/dt1".try_into().unwrap());

        assert_eq!(
            packed_tag_ref.target(),
            refs::Target::Peeled(hex_to_id("4c3f4cce493d7beb45012e478021b5f65295e5a3")),
            "it points to a tag object"
        );

        let object = packed_tag_ref.peel_to_oid_in_place().unwrap();
        let the_commit = hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03");
        assert_eq!(object, the_commit, "it is assumed to be fully peeled");
        assert_eq!(
            object,
            packed_tag_ref.peel_to_oid_in_place().unwrap(),
            "peeling again yields the same object"
        );

        let mut symbolic_ref = repo.find_reference("multi-link-target1").unwrap();
        assert_eq!(symbolic_ref.name(), "refs/heads/multi-link-target1".try_into().unwrap());
        assert_eq!(symbolic_ref.peel_to_oid_in_place().unwrap(), the_commit);
        assert_eq!(
            symbolic_ref.name(),
            "refs/remotes/origin/multi-link-target3".try_into().unwrap(),
            "it follows symbolic refs, too"
        );
        assert_eq!(symbolic_ref.peel_to_oid_in_place().unwrap(), the_commit, "idempotency");
    }
}
