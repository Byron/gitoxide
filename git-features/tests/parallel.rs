mod optimize_chunk_size_and_thread_limit {
    use git_features::parallel::optimize_chunk_size_and_thread_limit;

    #[test]
    fn not_enough_chunks_for_threads() {
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(10), None, Some(10)),
            (1, Some(5), 5)
        );
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(10), Some(3), Some(10)),
            (1, Some(3), 3),
            "the thread limit is always respected"
        );
    }

    #[test]
    fn some_more_chunks_per_thread() {
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(30), None, Some(10)),
            (1, Some(10), 10)
        );
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(30), Some(5), Some(10)),
            (3, Some(5), 5),
            "the thread limit is always respected"
        );
    }

    #[test]
    fn chunk_size_too_small() {
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(100), None, Some(10)),
            (5, Some(10), 10)
        );
        assert_eq!(
            optimize_chunk_size_and_thread_limit(1, Some(100), Some(5), Some(10)),
            (10, Some(5), 5),
            "the thread limit is always respected"
        );
    }

    #[test]
    fn chunk_size_too_big() {
        assert_eq!(
            optimize_chunk_size_and_thread_limit(50, Some(100), None, Some(10)),
            (5, Some(10), 10)
        );
        assert_eq!(
            optimize_chunk_size_and_thread_limit(50, Some(100), Some(5), Some(10)),
            (10, Some(5), 5),
            "the thread limit is always respected"
        );
    }

    mod unknown_chunk_count {
        use git_features::parallel::optimize_chunk_size_and_thread_limit;

        #[test]
        fn medium_chunk_size_many_threads() {
            assert_eq!(
                optimize_chunk_size_and_thread_limit(50, None, None, Some(4)),
                (50, Some(4), 4),
                "really, what do we know"
            );
        }

        #[test]
        fn medium_chunk_size_single_thread() {
            assert_eq!(
                optimize_chunk_size_and_thread_limit(50, None, None, Some(1)),
                (50, Some(1), 1),
                "single threaded - we don't touch that"
            );
        }

        #[test]
        fn small_chunk_size_single_thread() {
            assert_eq!(
                optimize_chunk_size_and_thread_limit(1, None, None, Some(1)),
                (1, Some(1), 1),
                "single threaded - we don't touch that"
            );
        }

        #[test]
        fn small_chunk_size_many_threads() {
            assert_eq!(
                    optimize_chunk_size_and_thread_limit(1, None, None, Some(4)),
                    (50, Some(4), 4),
                    "we prefer an arbitrary number, which should really be based on effort, but the caller has to adjust for that"
                );
        }
    }

    mod real_values {
        use git_features::parallel::optimize_chunk_size_and_thread_limit;

        #[test]
        fn linux_kernel_pack_my_machine_lookup() {
            assert_eq!(
                optimize_chunk_size_and_thread_limit(10000, Some(7_500_000), None, Some(4)),
                (1000, Some(4), 4),
                "the bucket size is capped actually, somewhat arbitrarily"
            );
        }

        #[test]
        fn linux_kernel_pack_my_machine_indexed() {
            assert_eq!(
                optimize_chunk_size_and_thread_limit(1, None, None, Some(4)),
                (50, Some(4), 4),
                "low values are raised to arbitrary value"
            );
            assert_eq!(
                optimize_chunk_size_and_thread_limit(10000, None, None, Some(4)),
                (1000, Some(4), 4),
                "high values are capped"
            );
        }
    }
}
