mod graph {
    mod commit {
        #[test]
        fn size_of_commit() {
            assert_eq!(
                std::mem::size_of::<gix_revwalk::graph::Commit<()>>(),
                48,
                "We might see quite a lot of these, so they shouldn't grow unexpectedly"
            )
        }
    }
}
