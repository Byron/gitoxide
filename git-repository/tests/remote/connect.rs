#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    mod protocol_allow {
        use crate::remote;
        use git_features::progress;
        use git_repository::remote::Direction::Fetch;
        use serial_test::serial;

        #[test]
        #[ignore]
        fn deny() {
            for name in ["protocol_denied", "protocol_file_denied"] {
                let repo = remote::repo(name);
                let remote = repo.find_remote("origin").unwrap();
                assert_eq!(
                    remote
                        .connect(Fetch, progress::Discard)
                        .err()
                        .map(|err| err.to_string())
                        .unwrap(),
                    "a nice error message or maybe match against it"
                );
            }
        }

        #[test]
        #[ignore]
        #[serial]
        fn user() {
            for (env_value, should_allow) in [(None, true), (Some("0"), false), (Some("1"), true)] {
                let _env = env_value.map(|value| git_testtools::Env::new().set("GIT_PROTOCOL_FROM_USER", value));
                let repo = remote::repo("protocol_file_user");
                let remote = repo.find_remote("origin").unwrap();
                assert_eq!(remote.connect(Fetch, progress::Discard).is_ok(), should_allow);
            }
        }
    }
}
