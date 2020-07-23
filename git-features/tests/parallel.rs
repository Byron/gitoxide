mod optimize_chunk_size_and_thread_limit {
    use git_features::parallel::optimize_chunk_size_and_thread_limit;

    #[test]
    fn not_enough_chunks_for_threads() {
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(10), None, Some(10)),
            (1, Some(5))
        );
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(10), Some(3), Some(10)),
            (1, Some(3)),
            "the thread limit is always respected"
        );
    }

    #[test]
    fn way_more_chunks_per_thread() {
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(30), None, Some(10)),
            (1, Some(10))
        );
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(30), Some(5), Some(10)),
            (1, Some(5)),
            "the thread limit is always respected"
        );
    }
}
