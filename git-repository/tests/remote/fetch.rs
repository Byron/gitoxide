#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use git_features::progress;
    use git_repository as git;
    use git_repository::remote::fetch;
    use git_repository::remote::Direction::Fetch;
    use git_testtools::hex_to_id;
    use std::sync::atomic::AtomicBool;

    pub(crate) fn repo_rw(name: &str) -> (git::Repository, git_testtools::tempfile::TempDir) {
        let dir = git_testtools::scripted_fixture_repo_writable_with_args(
            "make_fetch_repos.sh",
            &[] as &[String],
            git_testtools::Creation::ExecuteScript,
        )
        .unwrap();
        let repo = git::open_opts(dir.path().join(name), git::open::Options::isolated()).unwrap();
        (repo, dir)
    }

    #[test]
    fn fetch_pack() -> crate::Result {
        for version in [
            None,
            Some(git::protocol::transport::Protocol::V2),
            Some(git::protocol::transport::Protocol::V1),
        ] {
            let (mut repo, _tmp) = repo_rw("two-origins");
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
                        assert_eq!(write_pack_bundle.index.num_objects, 3, "this value is 4 when git does it with 'consecutive' negotiation style, but could be 33 if completely naive.");
                        assert_eq!(
                            write_pack_bundle.index.index_version,
                            git::odb::pack::index::Version::V2
                        );
                        assert_eq!(
                            write_pack_bundle.index.index_hash,
                            hex_to_id("5e0c69c18bf1835edaa103622dc8637fd87ea2f3")
                        );
                    }
                    fetch::Status::NoChange => unreachable!("we firmly expect changes here"),
                }
            }
        }
        Ok(())
    }
}
