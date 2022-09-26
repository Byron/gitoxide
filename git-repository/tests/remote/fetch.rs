#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use git_features::progress;
    use git_repository as git;
    use git_repository::remote::fetch;
    use git_repository::remote::Direction::Fetch;
    use git_testtools::hex_to_id;
    use std::sync::atomic::AtomicBool;

    use crate::remote;

    #[test]
    fn fetch_pack() -> crate::Result {
        for version in [
            None,
            Some(git::protocol::transport::Protocol::V2),
            Some(git::protocol::transport::Protocol::V1),
        ] {
            let (mut repo, _tmp) = remote::repo_rw("two-origins");
            if let Some(version) = version {
                repo.config_snapshot_mut().set_raw_value(
                    "protocol",
                    None,
                    "version",
                    (version as u8).to_string().as_str(),
                )?;
            }

            // No updates
            {
                let remote = repo.find_remote("origin")?;
                {
                    remote
                        .connect(Fetch, progress::Discard)?
                        .prepare_fetch(Default::default())?;
                    // early drops are fine and won't block.
                }
                let outcome = remote
                    .connect(Fetch, progress::Discard)?
                    .prepare_fetch(Default::default())?
                    .receive(&AtomicBool::default())?;
                assert!(matches!(outcome.status, git::remote::fetch::Status::NoChange));
            }

            // Some updates to be fetched
            {
                let remote = repo.find_remote("changes-on-top-of-origin")?;
                let outcome: git::remote::fetch::Outcome = remote
                    .connect(Fetch, progress::Discard)?
                    .prepare_fetch(Default::default())?
                    .receive(&AtomicBool::default())?;
                match outcome.status {
                    fetch::Status::Change { write_pack_bundle } => {
                        assert_eq!(write_pack_bundle.pack_kind, git::odb::pack::data::Version::V2);
                        assert_eq!(write_pack_bundle.object_hash, repo.object_hash());
                        assert_eq!(write_pack_bundle.index.num_objects, 33); // TODO: should just be 4! but in naive mode it's what happens currently.
                        assert_eq!(
                            write_pack_bundle.index.index_version,
                            git::odb::pack::index::Version::V2
                        );
                        assert_eq!(
                            write_pack_bundle.index.index_hash,
                            hex_to_id("1e396d0e2ab415556b240dc6251c65c71b568caa")
                        );
                    }
                    fetch::Status::NoChange => unreachable!("we firmly expect changes here"),
                }
            }
        }
        Ok(())
    }
}
