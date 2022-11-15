#[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
mod blocking_and_async_io {
    use git_features::progress;
    use git_repository as git;
    use git_repository::remote::Direction::Fetch;

    use crate::remote;
    use crate::remote::{into_daemon_remote_if_async, spawn_git_daemon_if_async};
    use git_protocol::maybe_async;

    #[maybe_async::test(
        feature = "blocking-network-client",
        async(feature = "async-network-client-async-std", async_std::test)
    )]
    async fn all() -> crate::Result {
        let daemon = spawn_git_daemon_if_async(remote::repo_path("base"))?;
        for (version, expected_remote_refs) in [
            (None, 11),
            (Some(git::protocol::transport::Protocol::V2), 11),
            (Some(git::protocol::transport::Protocol::V1), 14), // V1 doesn't support prefiltering.
        ] {
            let mut repo = remote::repo("clone");
            if let Some(version) = version {
                repo.config_snapshot_mut().set_raw_value(
                    "protocol",
                    None,
                    "version",
                    (version as u8).to_string().as_str(),
                )?;
            }

            let remote = into_daemon_remote_if_async(repo.find_remote("origin")?, daemon.as_ref(), None);
            let map = remote
                .connect(Fetch, progress::Discard)
                .await?
                .ref_map(Default::default())
                .await?;
            assert_eq!(
                map.remote_refs.len(),
                expected_remote_refs,
                "{:?}: it gets all remote refs, independently of the refspec. But we use a prefix so pre-filter them.",
                version
            );

            assert_eq!(map.fixes.len(), 0);
            assert_eq!(
                map.mappings.len(),
                11,
                "mappings are only a sub-set of all remotes due to refspec matching, tags are filtered out."
            );
        }
        Ok(())
    }
}
