mod write_object {
    use git_repository::prelude::ObjectAccessExt;

    #[test]
    fn empty_tree() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        let repo = git_repository::init_bare(&tmp)?.into_easy();
        let oid = repo.write_object(&git_repository::objs::Tree::empty().into())?;
        assert_eq!(
            oid,
            git_repository::hash::ObjectId::empty_tree(),
            "it produces a well-known empty tree id"
        );
        Ok(())
    }
}

mod commit {
    use git_repository as git;
    use git_repository::prelude::{ObjectAccessExt, ReferenceAccessExt};
    use git_testtools::hex_to_id;

    #[test]
    fn single_line_initial_commit_empty_tree_ref_nonexisting() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = git::init_bare(&tmp).unwrap().into_easy();
        let empty_tree_id = repo.write_object(&git::objs::Tree::empty().into()).unwrap();
        let author = git::actor::Signature::empty();
        let commit_id = repo
            .commit(
                "HEAD",
                "initial",
                author.clone(),
                author,
                empty_tree_id,
                git::commit::NO_PARENT_IDS,
            )
            .unwrap();
        assert_eq!(
            commit_id,
            hex_to_id("302ea5640358f98ba23cda66c1e664a6f274643f"),
            "the commit id is stable"
        );

        // TODO: check reflog
    }

    #[test]
    #[ignore]
    fn multi_line_commit_message_uses_first_line_in_ref_log_ref_nonexisting() {
        let (repo, _keep) = crate::basic_rw_repo().unwrap();
        let parent = repo.find_reference("HEAD").unwrap().peel_to_oid_in_place().unwrap();
        let empty_tree_id = parent
            .object()
            .unwrap()
            .commit_iter()
            .tree_id()
            .expect("tree to be set");
        let author = git::actor::Signature::empty();
        let commit_id = repo
            .commit(
                "HEAD",
                "hello there \r\n\nthe body",
                author.clone(),
                author.clone(),
                empty_tree_id,
                Some(parent),
            )
            .unwrap();
        assert_eq!(
            commit_id,
            hex_to_id("1ff7decccf76bfa15bfdb0b66bac0c9144b4b083"),
            "the commit id is stable"
        );

        let commit_id = repo
            .commit(
                "refs/heads/new-branch",
                "committing into a new branch creates it",
                author.clone(),
                author,
                empty_tree_id,
                Some(commit_id),
            )
            .unwrap();

        assert_eq!(
            commit_id,
            hex_to_id("1ff7decccf76bfa15bfdb0b66bac0c9144b4b083"),
            "the second commit id is stable"
        );

        // TODO: check reflog
        let branch = repo.head().unwrap().expect("head is not detached");
        let current_commit = branch.target().as_id().expect("peeled").to_owned();
        assert_eq!(current_commit, commit_id.detach(), "the commit was set");
    }
}
