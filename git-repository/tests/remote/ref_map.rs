#[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
mod blocking_and_async_io {
    use git_features::progress;
    use git_protocol::maybe_async;
    use git_repository as git;
    use git_repository::remote::Direction::Fetch;

    use crate::{
        remote,
        remote::{into_daemon_remote_if_async, spawn_git_daemon_if_async},
    };

    #[maybe_async::test(
        feature = "blocking-network-client",
        async(feature = "async-network-client-async-std", async_std::test)
    )]
    async fn all() -> crate::Result {
        let daemon = spawn_git_daemon_if_async(remote::repo_path("base"))?;
        for (fetch_tags, version, expected_remote_refs, expected_mappings) in [
            (git::remote::fetch::Tags::None, None, 11, 11),
            (
                git::remote::fetch::Tags::None,
                Some(git::protocol::transport::Protocol::V2),
                11,
                11,
            ),
            (
                git::remote::fetch::Tags::Included,
                Some(git::protocol::transport::Protocol::V2),
                17,
                17,
            ),
            (
                git::remote::fetch::Tags::All,
                Some(git::protocol::transport::Protocol::V2),
                17,
                17,
            ),
            (
                git::remote::fetch::Tags::None,
                Some(git::protocol::transport::Protocol::V1),
                18,
                11,
            ),
            (
                git::remote::fetch::Tags::Included,
                Some(git::protocol::transport::Protocol::V1),
                18,
                17,
            ),
            (
                git::remote::fetch::Tags::All,
                Some(git::protocol::transport::Protocol::V1),
                18,
                17,
            ),
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

            let remote = into_daemon_remote_if_async(
                repo.find_remote("origin")?.with_fetch_tags(fetch_tags),
                daemon.as_ref(),
                None,
            );
            let map = remote
                .connect(Fetch, progress::Discard)
                .await?
                .ref_map(Default::default())
                .await?;
            assert_eq!(
                    map.remote_refs.len(),
                    expected_remote_refs ,
                    "{:?} fetch-tags={fetch_tags:?}: it gets all remote refs, independently of the refspec. But we use a prefix so pre-filter them.",
                    version
                );

            assert_eq!(map.fixes.len(), 0);
            assert_eq!(
                map.mappings.len(),
                expected_mappings,
                "mappings are only a sub-set of all remotes due to refspec matching, tags are filtered out."
            );
        }
        Ok(())
    }
}
