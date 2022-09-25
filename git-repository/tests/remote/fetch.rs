#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use git_features::progress;
    use git_repository as git;
    use git_repository::remote::Direction::Fetch;
    use std::sync::atomic::AtomicBool;

    use crate::remote;

    #[test]
    fn fetch_pack() -> crate::Result {
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
            {
                remote
                    .connect(Fetch, progress::Discard)?
                    .prepare_fetch(Default::default())?;
                // early drops are fine and won't block.
            }
            // TODO: make sure there is actually something to do to run into other issues
            let outcome = remote
                .connect(Fetch, progress::Discard)?
                .prepare_fetch(Default::default())?
                .receive(&AtomicBool::default())?;
            assert!(matches!(outcome.status, git::remote::fetch::Status::NoChange));
        }
        Ok(())
    }
}
