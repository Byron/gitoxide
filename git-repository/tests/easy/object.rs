mod commit {
    use git_repository::{Commit, Repository};
    use git_testtools::hex_to_id;

    use crate::basic_repo;

    #[test]
    fn tree() {
        let handle = basic_repo().unwrap();
        let commit = head_commit(&handle);

        assert_eq!(commit.tree().unwrap().id, commit.tree_id().expect("id present"));
        assert_eq!(
            commit.tree_id(),
            Some(hex_to_id("21d3ba9a26b790a4858d67754ae05d04dfce4d0c"))
        )
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

    fn head_commit(handle: &Repository) -> Commit<'_> {
        handle
            .head()
            .unwrap()
            // TODO: Add something like peel_to_commit() to cut the chain, deal with unborn as Error
            .into_fully_peeled_id()
            .expect("born")
            .unwrap()
            .object()
            .unwrap()
            .into_commit()
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
