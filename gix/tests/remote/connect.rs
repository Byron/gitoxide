#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    mod protocol_allow {
        use gix::remote::Direction::Fetch;
        use gix_features::progress;
        use serial_test::serial;

        use crate::remote;

        #[test]
        fn deny() {
            for name in ["protocol_denied", "protocol_file_denied"] {
                let repo = remote::repo(name);
                let remote = repo.find_remote("origin").unwrap();
                assert!(matches!(
                    remote.connect(Fetch, progress::Discard).err(),
                    Some(gix::remote::connect::Error::ProtocolDenied {
                        url: _,
                        scheme: gix::url::Scheme::File
                    })
                ));
            }
        }

        #[test]
        #[serial]
        fn user() -> crate::Result {
            for (env_value, should_allow) in [(None, true), (Some("0"), false), (Some("1"), true)] {
                let _env = env_value.map(|value| gix_testtools::Env::new().set("GIT_PROTOCOL_FROM_USER", value));
                let repo = gix::open_opts(
                    remote::repo("protocol_file_user").git_dir(),
                    gix::open::Options::isolated().permissions(gix::Permissions {
                        env: gix::permissions::Environment {
                            git_prefix: gix_sec::Permission::Allow,
                            ..gix::permissions::Environment::all()
                        },
                        ..gix::Permissions::isolated()
                    }),
                )?;
                let remote = repo.find_remote("origin")?;
                assert_eq!(
                    remote.connect(Fetch, progress::Discard).is_ok(),
                    should_allow,
                    "Value = {:?}",
                    env_value
                );
            }
            Ok(())
        }
    }
}
