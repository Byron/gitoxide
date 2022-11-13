#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    mod protocol_allow {
        use git_features::progress;
        use git_repository as git;
        use git_repository::remote::Direction::Fetch;
        use serial_test::serial;

        use crate::remote;

        #[test]
        fn deny() {
            for name in ["protocol_denied", "protocol_file_denied"] {
                let repo = remote::repo(name);
                let remote = repo.find_remote("origin").unwrap();
                assert!(matches!(
                    remote.connect(Fetch, progress::Discard).err(),
                    Some(git::remote::connect::Error::ProtocolDenied {
                        url: _,
                        scheme: git::url::Scheme::File
                    })
                ));
            }
        }

        #[test]
        #[serial]
        fn user() -> crate::Result {
            for (env_value, should_allow) in [(None, true), (Some("0"), false), (Some("1"), true)] {
                let _env = env_value.map(|value| git_testtools::Env::new().set("GIT_PROTOCOL_FROM_USER", value));
                let repo = git::open_opts(
                    remote::repo("protocol_file_user").git_dir(),
                    git::open::Options::isolated().permissions(git::Permissions {
                        env: git::permissions::Environment {
                            git_prefix: git_sec::Permission::Allow,
                            ..git::permissions::Environment::all()
                        },
                        ..git::Permissions::isolated()
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

#[cfg(feature = "async-network-client-async-std")]
mod async_io_via_async_std {
    use git_features::progress;
    use git_repository::remote::Direction::Fetch;

    use crate::remote;
    use crate::remote::into_daemon_remote;

    #[async_std::test]
    async fn connect_only() -> crate::Result {
        let repo = remote::repo("protocol_file_user");
        let daemon = git_testtools::spawn_git_daemon(repo.git_dir())?;
        let remote = into_daemon_remote(repo.find_remote("origin")?, &daemon.url, None);
        remote
            .connect(Fetch, progress::Discard)
            .await
            .expect("connection succeeds");
        Ok(())
    }
}
