mod describe {
    use crate::named_repo;
    use git_repository::commit::describe::SelectRef::{AllRefs, AllTags, AnnotatedTags};

    #[test]
    fn tags_are_sorted_by_date_and_lexigraphically() {
        let repo = named_repo("make_commit_describe_multiple_tags.sh").unwrap();
        let mut describe = repo.head_commit().unwrap().describe();
        for filter in &[AnnotatedTags, AllTags, AllRefs] {
            describe = describe.names(*filter);
            assert_eq!(describe.format().unwrap().to_string(), "v2", "{:?}", filter);
        }
    }
}
