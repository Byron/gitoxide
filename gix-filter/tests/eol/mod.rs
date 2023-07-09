mod stats {
    mod from_bytes {
        use gix_filter::eol;

        #[test]
        fn all() {
            let stats = eol::Stats::from_bytes(b"\n\r\nhi\rho\0\tanother line\nother\r\nmixed");
            assert_eq!(
                stats,
                eol::Stats {
                    null: 1,
                    lone_cr: 1,
                    lone_lf: 2,
                    crlf: 2,
                    printable: 27,
                    non_printable: 1,
                }
            );
            assert!(stats.is_binary());
        }
    }
}

pub(crate) mod convert_to_git;
mod convert_to_worktree;
