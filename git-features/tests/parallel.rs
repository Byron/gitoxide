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
    fn some_more_chunks_per_thread() {
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(30), None, Some(10)),
            (1, Some(10))
        );
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(30), Some(5), Some(10)),
            (3, Some(5)),
            "the thread limit is always respected"
        );
    }
    #[test]
    fn chunk_size_too_small() {
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(100), None, Some(10)),
            (5, Some(10))
        );
    }

    #[test]
    fn chunk_size_too_big() {
        assert_eq!(
            optimize_chunk_size_and_thread_limit(50, Some(2), None, Some(10)),
            (2, Some(10))
        );
    }
}
