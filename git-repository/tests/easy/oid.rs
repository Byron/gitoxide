mod ancestors {
    use git_traverse::commit;

    #[test]
    fn all() -> crate::Result {
        let repo = crate::repo("make_repo_with_fork_and_dates.sh")?.to_easy();
        let head = repo.head()?.into_fully_peeled_id().expect("born")?;
        let commits_graph_order = head.ancestors().all().collect::<Result<Vec<_>, _>>()?;
        assert_eq!(commits_graph_order.len(), 4, "need a specific amount of commits");

        let commits_by_commit_date = head
            .ancestors()
            .sorting(commit::Sorting::ByCommitterDate)
            .all()
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(
            commits_by_commit_date.len(),
            4,
            "need a specific amount of commits, ordering doesn't affect that"
        );
        assert_ne!(
            commits_by_commit_date, commits_graph_order,
            "these are ordered differently"
        );

        assert_eq!(
            head.ancestors().first_parent_only().all().count(),
            3,
            "It skips merges this way."
        );
        Ok(())
    }
}
