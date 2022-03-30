mod commit {
    use std::cmp::Ordering;

    use git_repository::{Commit, Repository};
    use git_testtools::hex_to_id;

    use crate::basic_repo;

    #[test]
    fn short_id() -> crate::Result {
        let handle = basic_repo()?;
        let commit = head_commit(&handle);
        assert_eq!(commit.short_id()?.cmp_oid(&commit.id), Ordering::Equal);
        Ok(())
    }

    #[test]
    fn tree() -> crate::Result {
        let handle = basic_repo()?;
        let commit = head_commit(&handle);

        assert_eq!(commit.tree()?.id, commit.tree_id().expect("id present"));
        assert_eq!(
            commit.tree_id().ok(),
            Some(hex_to_id("21d3ba9a26b790a4858d67754ae05d04dfce4d0c"))
        );
        Ok(())
    }

    #[test]
    fn decode() -> crate::Result {
        let handle = basic_repo()?;
        let commit = head_commit(&handle);
        assert_eq!(commit.decode()?.message, commit.message_raw()?);
        assert_eq!(commit.decode()?.message(), commit.message()?);
        assert_eq!(commit.decode()?.message, "c2\n");
        Ok(())
    }

    fn head_commit(repo: &Repository) -> Commit<'_> {
        repo.head().unwrap().peel_to_commit_in_place().unwrap()
    }
}

#[test]
fn object_ref_size_in_memory() {
    assert_eq!(
        std::mem::size_of::<git_repository::Object<'_>>(),
        56,
        "the size of this structure should not changed unexpectedly"
    )
}

#[test]
fn oid_size_in_memory() {
    assert_eq!(
        std::mem::size_of::<git_repository::Id<'_>>(),
        32,
        "the size of this structure should not changed unexpectedly"
    )
}
