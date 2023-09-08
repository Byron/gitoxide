#[cfg(feature = "revision")]
mod describe {
    use gix::commit::describe::SelectRef::{AllRefs, AllTags, AnnotatedTags};

    use crate::named_repo;

    #[test]
    fn tags_are_sorted_by_date_and_lexicographically() -> crate::Result {
        let repo = named_repo("make_commit_describe_multiple_tags.sh")?;
        let mut describe = repo.head_commit()?.describe();
        for filter in &[AnnotatedTags, AllTags, AllRefs] {
            describe = describe.names(*filter);
            assert_eq!(describe.format()?.to_string(), "v4", "{filter:?}");
        }
        Ok(())
    }

    #[test]
    fn tags_are_sorted_by_priority() -> crate::Result {
        let repo = named_repo("make_commit_describe_multiple_tags.sh")?;
        let commit = repo.find_reference("refs/tags/v0")?.id().object()?.into_commit();
        let mut describe = commit.describe();
        for filter in &[AnnotatedTags, AllTags, AllRefs] {
            describe = describe.names(*filter);
            assert_eq!(describe.format()?.to_string(), "v1", "{filter:?}");
        }
        Ok(())
    }

    #[test]
    fn lightweight_tags_are_sorted_lexicographically() -> crate::Result {
        let repo = named_repo("make_commit_describe_multiple_tags.sh")?;
        let commit = repo.find_reference("refs/tags/l0")?.id().object()?.into_commit();
        let mut describe = commit.describe();
        for filter in &[AnnotatedTags, AllTags, AllRefs] {
            describe = describe.names(*filter);
            let expected = match filter {
                AnnotatedTags => None,
                _ => Some("l0"),
            };
            let actual = describe.try_format()?.map(|f| f.to_string());
            assert_eq!(actual.as_deref(), expected, "{filter:?}");
        }
        Ok(())
    }
}
