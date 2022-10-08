#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use git_features::progress;
    use git_repository as git;
    use git_repository::remote::Direction::Fetch;

    use crate::remote;

    #[test]
    fn all() -> crate::Result {
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

            let remote = repo.find_remote("origin")?;
            let map = remote.connect(Fetch, progress::Discard)?.ref_map(Default::default())?;
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
