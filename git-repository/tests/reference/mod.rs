use crate::repo_rw;

mod log {
    use git_repository as git;

    #[test]
    fn message() {
        assert_eq!(
            git::reference::log::message("commit", "the subject\n\nthe body".into(), 0),
            "commit (initial): the subject"
        );
        assert_eq!(
            git::reference::log::message("other", "the subject".into(), 1),
            "other: the subject"
        );

        assert_eq!(
            git::reference::log::message("rebase", "the subject".into(), 2),
            "rebase (merge): the subject"
        );
    }
}
mod find {
    use std::convert::TryInto;

    use git_hash::hex_to_id;
    use git_ref as refs;
    use git_ref::FullNameRef;

    fn repo() -> crate::Result<git_repository::Repository> {
        crate::repo("make_references_repo.sh").map(Into::into)
    }

    #[test]
    fn and_peel() -> crate::Result {
        let repo = repo()?;
        let mut packed_tag_ref = repo.try_find_reference("dt1")?.expect("tag to exist");
        let expected: &FullNameRef = "refs/tags/dt1".try_into()?;
        assert_eq!(packed_tag_ref.name(), expected);

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

        let expected: &FullNameRef = "refs/heads/multi-link-target1".try_into()?;
        assert_eq!(symbolic_ref.name(), expected);
        assert_eq!(symbolic_ref.peel_to_id_in_place()?, the_commit);

        let expected: &FullNameRef = "refs/remotes/origin/multi-link-target3".try_into()?;
        assert_eq!(symbolic_ref.name(), expected, "it follows symbolic refs, too");
        assert_eq!(symbolic_ref.into_fully_peeled_id()?, the_commit, "idempotency");
        Ok(())
    }
}

#[test]
fn set_target_id() {
    let (repo, _tmp) = repo_rw("make_basic_repo.sh").unwrap();
    let mut head_ref = repo.head_ref().unwrap().expect("present");
    let target_id = repo.rev_parse_single(":/c1").unwrap();
    let prev_id = head_ref.id();
    assert_ne!(prev_id, target_id, "we don't point to the target id yet");
    head_ref.set_target_id(target_id, "reflog message").unwrap();
    assert_eq!(head_ref.id(), target_id, "the id was set and is observable right away");

    head_ref.delete().unwrap();
    assert!(head_ref
        .set_target_id(prev_id, "fails")
        .unwrap_err()
        .to_string()
        .starts_with("Reference \"refs/heads/main\" was supposed to exist"));
}

mod remote;
