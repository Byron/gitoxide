#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use crate::remote;
    use git_features::progress;
    use git_repository as git;
    use git_repository::remote::Direction::Fetch;

    #[test]
    #[ignore]
    fn all() -> crate::Result {
        for version in [
            None,
            Some(git::protocol::transport::Protocol::V2),
            Some(git::protocol::transport::Protocol::V1),
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
            let connection = remote.connect(Fetch, progress::Discard)?;
            let refs = connection.list_refs()?;
            assert_eq!(refs.len(), 14, "it gets all remote refs, independently of the refspec.");
        }
        Ok(())
    }
}
