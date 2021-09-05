mod write_object {
    use git_repository::prelude::{ConfigAccessExt, ObjectAccessExt};

    #[test]
    fn empty_tree() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        let repo = git_repository::init_bare(&tmp)?.into_easy();
        let oid = repo.write_object(&git_repository::objs::Tree::empty().into())?;
        assert_eq!(
            oid,
            git_repository::hash::ObjectId::empty_tree(repo.hash_kind()?),
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
    fn single_line_initial_commit_empty_tree_ref_nonexisting() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        let repo = git::init(&tmp)?.into_easy();
        let empty_tree_id = repo.write_object(&git::objs::Tree::empty().into())?;
        let author = git::actor::Signature::empty();
        let commit_id = repo.commit(
            "HEAD",
            "initial",
            author.clone(),
            author,
            empty_tree_id,
            git::commit::NO_PARENT_IDS,
        )?;
        assert_eq!(
            commit_id,
            hex_to_id("302ea5640358f98ba23cda66c1e664a6f274643f"),
            "the commit id is stable"
        );

        let head = repo.head()?.into_referent();
        assert_eq!(
            head.log()?
                .iter_rev()?
                .expect("log present")
                .next()
                .expect("one line")?
                .message,
            "commit (initial): initial"
        );
        Ok(())
    }

    #[test]
    fn multi_line_commit_message_uses_first_line_in_ref_log_ref_nonexisting() -> crate::Result {
        let (repo, _keep) = crate::basic_rw_repo()?;
        let parent = repo.find_reference("HEAD")?.peel_to_id_in_place()?;
        let empty_tree_id = parent.object()?.commit_iter().tree_id().expect("tree to be set");
        let author = git::actor::Signature::empty();
        let first_commit_id = repo.commit(
            "HEAD",
            "hello there \r\n\nthe body",
            author.clone(),
            author.clone(),
            empty_tree_id,
            Some(parent),
        )?;
        assert_eq!(
            first_commit_id,
            hex_to_id("1ff7decccf76bfa15bfdb0b66bac0c9144b4b083"),
            "the commit id is stable"
        );

        let head_log_entries: Vec<_> = repo
            .head()?
            .log()?
            .iter_rev()?
            .expect("log present")
            .map(Result::unwrap)
            .map(|l| l.message)
            .collect();
        assert_eq!(
            head_log_entries,
            vec!["commit: hello there", "commit: c2", "commit (initial): c1"],
            "we get the actual HEAD log, not the log of some reference"
        );
        let current_commit = repo.head()?.into_fully_peeled_id().expect("born")?;
        assert_eq!(current_commit, &*first_commit_id, "the commit was set");

        let second_commit_id = repo.commit(
            "refs/heads/new-branch",
            "committing into a new branch creates it",
            author.clone(),
            author,
            empty_tree_id,
            Some(first_commit_id),
        )?;

        assert_eq!(
            second_commit_id,
            hex_to_id("b0d041ade77e51d31c79c7147fb769336ccc77b1"),
            "the second commit id is stable"
        );

        let mut branch = repo.find_reference("new-branch")?;
        let current_commit = branch.peel_to_id_in_place()?;
        assert_eq!(current_commit, second_commit_id, "the commit was set");

        let mut log = branch.log()?;
        let mut log_iter = log.iter_rev()?.expect("log present");
        assert_eq!(
            log_iter.next().expect("one line")?.message,
            "commit: committing into a new branch creates it"
        );
        assert!(
            log_iter.next().is_none(),
            "there is only one log line in the new branch"
        );
        Ok(())
    }
}
