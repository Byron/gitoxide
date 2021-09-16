mod ancestors {
    use git_repository::prelude::ReferenceAccessExt;

    #[test]
    fn all() -> crate::Result {
        let repo = crate::basic_repo()?;
        assert_eq!(
            repo.head()?
                .try_into_fully_peeled_id()
                .expect("born")?
                .ancestors()?
                .all()
                .count(),
            2,
            "need a specific amount of commits"
        );
        Ok(())
    }
}
