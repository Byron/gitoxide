mod describe {
    use crate::named_repo;
    use git_repository::commit::describe::SelectRef::AnnotatedTags;

    #[test]
    fn tags_are_sorted_by_date_and_lexigraphically() {
        let repo = named_repo("make_commit_describe_multiple_tags.sh").unwrap();
        assert_eq!(
            repo.head_commit()
                .unwrap()
                .describe()
                .names(AnnotatedTags)
                .format()
                .unwrap()
                .name
                .expect("name set")
                .as_ref(),
            "v1"
        );
    }
}
