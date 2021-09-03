mod head {
    use git_repository::prelude::ReferenceAccessExt;
    use git_testtools::hex_to_id;

    #[test]
    #[ignore]
    fn symbolic() {
        let repo = crate::basic_repo().unwrap();
        let head = repo.head().unwrap().expect("HEAD is symbolic");
        assert_eq!(
            head.inner.target.into_id(),
            hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
        );
    }
}
