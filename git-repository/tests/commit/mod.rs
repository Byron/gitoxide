mod describe {
    use git_repository::commit::describe::SelectRef::{AllRefs, AllTags, AnnotatedTags};

    use crate::named_repo;

    #[test]
    fn tags_are_sorted_by_date_and_lexicographically() {
        let repo = named_repo("make_commit_describe_multiple_tags.sh").unwrap();
        let mut describe = repo.head_commit().unwrap().describe();
        for filter in &[AnnotatedTags, AllTags, AllRefs] {
            describe = describe.names(*filter);
            assert_eq!(describe.format().unwrap().to_string(), "v4", "{:?}", filter);
        }
    }

    #[test]
    fn tags_are_sorted_by_priority() {
        let repo = named_repo("make_commit_describe_multiple_tags.sh").unwrap();
        let commit = repo
            .find_reference("refs/tags/v0")
            .unwrap()
            .id()
            .object()
            .unwrap()
            .into_commit();
        let mut describe = commit.describe();
        for filter in &[AnnotatedTags, AllTags, AllRefs] {
            describe = describe.names(*filter);
            assert_eq!(describe.format().unwrap().to_string(), "v1", "{:?}", filter);
        }
    }

    #[test]
    fn lightweight_tags_are_sorted_lexicographically() {
        let repo = named_repo("make_commit_describe_multiple_tags.sh").unwrap();
        let commit = repo
            .find_reference("refs/tags/l0")
            .unwrap()
            .id()
            .object()
            .unwrap()
            .into_commit();
        let mut describe = commit.describe();
        for filter in &[AnnotatedTags, AllTags, AllRefs] {
            describe = describe.names(*filter);
            let expected = match filter {
                AnnotatedTags => None,
                _ => Some("l0"),
            };
            let actual = describe.try_format().unwrap().map(|f| f.to_string());
            assert_eq!(actual.as_deref(), expected, "{:?}", filter);
        }
    }
}
