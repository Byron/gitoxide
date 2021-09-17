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
