mod log {
    use git_repository as git;

    #[test]
    fn message() {
        let mut commit = git::objs::Commit {
            tree: git::hash::ObjectId::empty_tree(git_hash::Kind::Sha1),
            parents: Default::default(),
            author: Default::default(),
            committer: Default::default(),
            encoding: None,
            message: "the subject\n\nthe body".into(),
            extra_headers: vec![],
        };
        assert_eq!(
            git::reference::log::message("commit", &commit),
            "commit (initial): the subject"
        );
        commit.parents.push(git::hash::ObjectId::null_sha1());
        assert_eq!(git::reference::log::message("other", &commit), "other: the subject");

        commit.parents.push(git::hash::ObjectId::null_sha1());
        assert_eq!(
            git::reference::log::message("rebase", &commit),
            "rebase (merge): the subject"
        );
    }
}
